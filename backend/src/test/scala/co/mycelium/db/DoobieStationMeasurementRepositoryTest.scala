package co.mycelium.db

import cats.effect.IO
import cats.effect.kernel.Resource
import co.mycelium.adapters.db.{DoobieStationMeasurementRepository, DoobieStationRepository}
import co.mycelium.domain.{CheckinEvent, MeasurementPeriod, StationInsert, StationMeasurement}
import doobie.weaver.*
import weaver.*
import doobie.*
import doobie.implicits.*

import java.time.Instant
import java.util.UUID

object DoobieStationMeasurementRepositoryTest extends IOSuite with IOChecker {

  override type Res = Transactor[IO]

  val now     = Instant.parse("2025-07-29T00:10:00Z")
  val insert  = StationInsert("00:00:00:00:00:00", "Unnamed")
  val station = insert.toStation(UUID.randomUUID(), now, "some-user-id")

  override def sharedResource: Resource[IO, Res] =
    DoobieResource.setup

  test("average should work") { implicit tx =>
    val bucket = java.time.Instant.now().truncatedTo(java.time.temporal.ChronoUnit.DAYS)

    val program = for {
      id <- DoobieStationRepository.insert(station, bucket)
      _  <- DoobieStationMeasurementRepository.insertMany(
        id,
        List(
          CheckinEvent.Measurement(bucket, None, 1250, 100.0, 20.5, 65.0, 25.0),
          CheckinEvent.Measurement(bucket.plusSeconds(60), None, 1260, 110.0, 21.0, 64.5, 26.0),
          CheckinEvent.Measurement(bucket.plusSeconds(120), None, 1270, 120.0, 21.5, 64.0, 27.0),
          CheckinEvent.Measurement(bucket.plusSeconds(180), None, 1280, 130.0, 22.0, 63.5, 28.0),
          CheckinEvent.Measurement(bucket.plusSeconds(240), None, 1290, 140.0, 22.5, 63.0, 29.0),
          CheckinEvent.Measurement(bucket.plusSeconds(300), None, 1300, 150.0, 23.0, 62.5, 30.0),
          CheckinEvent.Measurement(bucket.plusSeconds(360), None, 1310, 160.0, 23.5, 62.0, 31.0),
          CheckinEvent.Measurement(bucket.plusSeconds(420), None, 1320, 170.0, 24.0, 61.5, 32.0),
          CheckinEvent.Measurement(bucket.plusSeconds(480), None, 1330, 180.0, 24.5, 61.0, 33.0),
          CheckinEvent.Measurement(bucket.plusSeconds(540), None, 1340, 190.0, 25.0, 60.5, 34.0)
        )
      )
      avg <- DoobieStationMeasurementRepository.avg(id, MeasurementPeriod.LastMonth)
    } yield {
      expect.eql(avg, List(StationMeasurement(bucket, 1295, 25.0, 62.75, 190.0, 29.5)))
    }

    program.transact(tx)
  }

}
