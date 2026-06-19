package co.mycelium.adapters.db

import co.mycelium.domain.{CheckinEvent, MeasurementPeriod, StationWatering}
import co.mycelium.ports.StationWateringRepository
import doobie.*
import doobie.implicits.*
import doobie.postgres.implicits.*

import java.util.UUID

object DoobieStationWateringRepository extends StationWateringRepository[ConnectionIO] {
  override def insertMany(
      stationId: UUID,
      waterings: List[CheckinEvent.Watering]
  ): ConnectionIO[Int] =
    Update[(UUID, java.time.Instant, Long)](
      "insert into station_waterings (station_id, occurred_at, duration_msec) values (?, ?, ?)"
    )
      .updateMany(waterings.map(w => (stationId, w.occurredAt, w.durationMsec)))

  override def listByPeriod(
      stationId: UUID,
      period: MeasurementPeriod
  ): ConnectionIO[List[StationWatering]] = {
    val lookback = MeasurementPeriod.lookbackInterval(period)
    (fr"SELECT occurred_at, duration_msec FROM station_waterings WHERE station_id = $stationId AND occurred_at >= now() - interval " ++
      Fragment.const(s"'$lookback'") ++
      fr" ORDER BY occurred_at ASC")
      .query[StationWatering]
      .to[List]
  }
}
