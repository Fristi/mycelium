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
    val timebucket = Instant.parse("2025-07-29T00:00:00Z")

    val program = for {
      id <- DoobieStationRepository.insert(station, now)
      _  <- DoobieStationMeasurementRepository.insertMany(
        id,
        List(
          CheckinEvent.Measurement(now, None, 1250, 100.0, 20.5, 65.0, 2.5),
          CheckinEvent.Measurement(now.plusSeconds(60), None, 1260, 110.0, 21.0, 64.5, 2.6),
          CheckinEvent.Measurement(now.plusSeconds(120), None, 1270, 120.0, 21.5, 64.0, 2.7),
          CheckinEvent.Measurement(now.plusSeconds(180), None, 1280, 130.0, 22.0, 63.5, 2.8),
          CheckinEvent.Measurement(now.plusSeconds(240), None, 1290, 140.0, 22.5, 63.0, 2.9),
          CheckinEvent.Measurement(now.plusSeconds(300), None, 1300, 150.0, 23.0, 62.5, 3.0),
          CheckinEvent.Measurement(now.plusSeconds(360), None, 1310, 160.0, 23.5, 62.0, 3.1),
          CheckinEvent.Measurement(now.plusSeconds(420), None, 1320, 170.0, 24.0, 61.5, 3.2),
          CheckinEvent.Measurement(now.plusSeconds(480), None, 1330, 180.0, 24.5, 61.0, 3.3),
          CheckinEvent.Measurement(now.plusSeconds(540), None, 1340, 190.0, 25.0, 60.5, 3.4)
        )
      )
      avg <- DoobieStationMeasurementRepository.avg(id, MeasurementPeriod.LastMonth)
    } yield {
      expect.eql(avg, List(StationMeasurement(timebucket, 1295, 22.75, 62.75, 145.0, 2.95)))
    }

    program.transact(tx)
  }

}
