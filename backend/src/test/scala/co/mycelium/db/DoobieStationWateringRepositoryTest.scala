package co.mycelium.db

import cats.effect.IO
import cats.effect.kernel.Resource
import co.mycelium.adapters.db.{DoobieStationRepository, DoobieStationWateringRepository}
import co.mycelium.domain.{CheckinEvent, MeasurementPeriod, StationInsert}
import doobie.weaver.*
import weaver.*
import doobie.*
import doobie.implicits.*
import doobie.postgres.implicits.*

import java.time.Instant
import java.util.UUID

object DoobieStationWateringRepositoryTest extends IOSuite with IOChecker {

  override type Res = Transactor[IO]

  val now     = Instant.parse("2025-07-29T00:10:00Z")
  val insert  = StationInsert("00:00:00:00:00:00", "Unnamed")
  val station = insert.toStation(UUID.randomUUID(), now, "some-user-id")

  override def sharedResource: Resource[IO, Res] =
    DoobieResource.setup

  test("insertMany should insert watering events") { implicit tx =>
    val program = for {
      id    <- DoobieStationRepository.insert(station, now)
      count <- DoobieStationWateringRepository.insertMany(
        id,
        List(
          CheckinEvent.Watering(now, 5000L),
          CheckinEvent.Watering(now.plusSeconds(3600), 3000L)
        )
      )
      rows <- fr"SELECT count(*) FROM station_waterings WHERE station_id = $id"
        .query[Int]
        .unique
    } yield {
      expect.eql(count, 2) && expect.eql(rows, 2)
    }

    program.transact(tx)
  }

  test("listByPeriod should return waterings within the period window") { implicit tx =>
    val program = for {
      id <- DoobieStationRepository.insert(station, now)
      _ <- DoobieStationWateringRepository.insertMany(
        id,
        List(
          CheckinEvent.Watering(now, 5000L),
          CheckinEvent.Watering(now.plusSeconds(3600), 3000L)
        )
      )
      waterings <- DoobieStationWateringRepository.listByPeriod(id, MeasurementPeriod.LastTwentyFourHours)
    } yield {
      expect.eql(waterings.length, 2) &&
      expect.eql(waterings.head.durationMsec, 5000L) &&
      expect.eql(waterings(1).durationMsec, 3000L)
    }

    program.transact(tx)
  }

}
