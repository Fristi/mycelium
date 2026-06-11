package co.mycelium.db

import cats.effect.IO
import cats.effect.kernel.Resource
import co.mycelium.adapters.db.{DoobieStationRepository, DoobieStationWateringRepository}
import co.mycelium.domain.{CheckinEvent, StationInsert}
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

}
