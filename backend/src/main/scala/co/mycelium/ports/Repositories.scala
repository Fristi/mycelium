package co.mycelium.ports

import cats.effect.kernel.MonadCancelThrow
import cats.tagless.{Derive, FunctorK}
import co.mycelium.adapters.db.DoobieRepositories
import co.mycelium.domain.*
import doobie.{Transactor}

import java.time.Instant
import java.util.UUID
import scala.annotation.experimental

trait StationProfileRepository[F[_]] {
  def upsert(profile: PlantProfile, stationId: UUID): F[Int]
  def getPlantProfilesByUserId(userId: String): F[List[StationPlantProfile]]
}

object StationProfileRepository {
  implicit val functorK: FunctorK[StationProfileRepository] = Derive.functorK
}

trait StationWateringRepository[F[_]] {
  def insertMany(stationId: UUID, waterings: List[CheckinEvent.Watering]): F[Int]
}

object StationWateringRepository {
  implicit val functorK: FunctorK[StationWateringRepository] = Derive.functorK
}

trait StationMeasurementRepository[F[_]] {
  def insertMany(stationId: UUID, measurements: List[CheckinEvent.Measurement]): F[Int]

  def avg(stationId: UUID, period: MeasurementPeriod): F[List[StationMeasurement]]
}

object StationMeasurementRepository {
  implicit val functorK: FunctorK[StationMeasurementRepository] = Derive.functorK
}

trait StationRepository[F[_]] {
  def insert(station: Station, on: Instant): F[UUID]
  def listByUserId(userId: String): F[List[Station]]
  def findById(id: UUID, userId: String): F[Option[Station]]
  def delete(id: UUID, userId: String): F[Int]
  def update(id: UUID, userId: String, update: StationUpdate, now: Instant): F[Int]
}

object StationRepository {
  implicit val functorK: FunctorK[StationRepository] = Derive.functorK
}

trait Repositories[F[_]] {
  def waterings: StationWateringRepository[F]
  def stationProfile: StationProfileRepository[F]
  def stations: StationRepository[F]
  def measurements: StationMeasurementRepository[F]
}

object Repositories {
  implicit val functorK: FunctorK[Repositories] = Derive.functorK

  def fromTransactor[F[_]: MonadCancelThrow](transactor: Transactor[F]): Repositories[F] =
    FunctorK[Repositories].mapK(DoobieRepositories)(transactor.trans)
}
