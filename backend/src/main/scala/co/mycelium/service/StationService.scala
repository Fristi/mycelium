package co.mycelium.service

import blobstore.s3.S3Store
import blobstore.url.{Authority, Url}
import cats.Monad
import cats.effect.std.UUIDGen
import cats.effect.{Clock, Concurrent}
import cats.implicits.*
import co.mycelium.domain.*
import co.mycelium.ports.*
import fs2.Stream

import java.util.UUID

trait StationService[F[_]] {
  def add(userId: String, station: StationInsert): F[UUID]
  def list(userId: String): F[List[Station]]
  def delete(userId: String, stationID: UUID): F[Unit]
  def update(userId: String, stationID: UUID, update: StationUpdate): F[Unit]
  def checkin(userId: String, stationID: UUID, events: List[CheckinEvent]): F[Int]
  def details(
      userId: String,
      period: Option[MeasurementPeriod],
      stationID: UUID
  ): F[Either[Unit, StationDetails]]
  def uploadAvatar(
      userId: String,
      stationID: UUID,
      image: Stream[F, Byte]
  ): F[List[PlantProfile]]
  def viewAvatar(stationID: UUID): F[Stream[F, Byte]]
  def getProfiles(userId: String): F[List[StationPlantProfile]]
  def setProfile(userId: String, stationID: UUID, profile: PlantProfile): F[Unit]
}

final class StationServiceImpl[F[_]: {Concurrent}](
    uuidGen: UUIDGen[F],
    clock: Clock[F],
    repos: Repositories[F],
    s3: S3Store[F],
    plantClassifier: PlantClassifier[F],
    plantProfiler: PlantProfiler[F],
    plantAvatarPlaceHolder: PlantAvatarPlaceHolder[F]
) extends StationService[F] {

  override def checkin(
      userId: String,
      stationID: UUID,
      events: List[CheckinEvent]
  ): F[Int] =
    protect(userId, stationID) {
      val measurements = events.collect { case m: CheckinEvent.Measurement => m }
      val waterings    = events.collect { case w: CheckinEvent.Watering    => w }
      for {
        measurementCount <- repos.measurements.insertMany(stationID, measurements)
        wateringCount    <- repos.waterings.insertMany(stationID, waterings)
      } yield measurementCount + wateringCount
    }

  override def add(userId: String, insert: StationInsert): F[UUID] =
    for {
      id      <- uuidGen.randomUUID
      created <- clock.realTimeInstant
      station = insert.toStation(id, created, userId)
      stationId <- repos.stations.insert(station, created)
    } yield stationId

  override def list(userId: String): F[List[Station]] =
    repos.stations.listByUserId(userId)

  override def delete(userId: String, stationID: UUID): F[Unit] =
    repos.stations.delete(stationID, userId).void

  override def update(userId: String, stationID: UUID, update: StationUpdate): F[Unit] =
    for {
      now <- clock.realTimeInstant
      _   <- repos.stations.update(stationID, userId, update, now)
    } yield ()

  override def details(
      userId: String,
      period: Option[MeasurementPeriod],
      stationID: UUID
  ): F[Either[Unit, StationDetails]] =
    repos.stations.findById(stationID, userId).flatMap {
      case Some(station) =>
        val effectivePeriod = period.getOrElse(MeasurementPeriod.LastTwentyFourHours)
        for {
          measurements <- repos.measurements.avg(stationID, effectivePeriod)
          waterings    <- repos.waterings.listByPeriod(stationID, effectivePeriod)
          profile      <- repos.stationProfile.getByStationId(stationID)
          growthPeriods = GrowthPeriodClassifier.classify(measurements, profile, effectivePeriod)
        } yield Right(StationDetails(station, measurements, waterings, growthPeriods))
      case None =>
        Monad[F].pure(Left(()))
    }

  private def mkUrl(uuid: UUID) =
    Url("s3", Authority.unsafe("mycelium"), blobstore.url.Path(s"$uuid"))

  override def uploadAvatar(
      userId: String,
      stationID: UUID,
      image: Stream[F, Byte]
  ): F[List[PlantProfile]] = {
    def image_ = image.observeAsync(256)(s3.put(mkUrl(stationID), true).andThen(_.drain))

    protect(userId, stationID) {
      for {
        possibleNames    <- plantClassifier.classifyPlant(image_)
        possibleProfiles <- plantProfiler.getProfilesForPlant(possibleNames)
      } yield possibleProfiles
    }
  }

  override def viewAvatar(stationID: UUID): F[Stream[F, Byte]] = {
    def bucket: Stream[F, Byte]      = s3.get(mkUrl(stationID))
    def placeHolder: Stream[F, Byte] = plantAvatarPlaceHolder.get

    Monad[F].pure(bucket.orElse(placeHolder))
  }

  override def getProfiles(userId: String): F[List[StationPlantProfile]] =
    repos.stationProfile.getPlantProfilesByUserId(userId)

  override def setProfile(userId: String, stationID: UUID, profile: PlantProfile): F[Unit] =
    protect(userId, stationID)(repos.stationProfile.upsert(profile, stationID).void)

  private def protect[A](userId: String, stationID: UUID)(f: => F[A]): F[A] =
    repos.stations.findById(stationID, userId).flatMap {
      case Some(_) => f
      case None    => Concurrent[F].raiseError(new Throwable("No access to this station"))
    }
}
