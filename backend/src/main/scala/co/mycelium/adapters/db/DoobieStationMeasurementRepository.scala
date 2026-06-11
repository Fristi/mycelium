package co.mycelium.adapters.db

import co.mycelium.domain.*
import co.mycelium.ports.StationMeasurementRepository
import doobie.*
import doobie.implicits.*
import doobie.postgres.implicits.*

import java.util.UUID

object DoobieStationMeasurementRepository extends StationMeasurementRepository[ConnectionIO] {
  override def insertMany(
      stationId: UUID,
      measurements: List[CheckinEvent.Measurement]
  ): ConnectionIO[Int] =
    Update[(UUID, java.time.Instant, Option[java.time.Instant], Int, Double, Double, Double, Double)](
      "insert into station_measurements (station_id, occurred_on, ended_on, battery, temperature, humidity, lux, soil_pf) values (?, ?, ?, ?, ?, ?, ?, ?)"
    )
      .updateMany(
        measurements.map(m =>
          (stationId, m.start, m.end, m.battery, m.temperature, m.humidity, m.lux, m.soilPf)
        )
      )

  override def avg(
      stationId: UUID,
      period: MeasurementPeriod
  ): ConnectionIO[List[StationMeasurement]] = {

    val timeBucket = period match {
      case MeasurementPeriod.LastTwentyFourHours => fr"time_bucket('15 minutes', occurred_on)"
      case MeasurementPeriod.LastSevenDays       => fr"time_bucket('1 day', occurred_on)"
      case MeasurementPeriod.LastTwoWeeks        => fr"time_bucket('1 day', occurred_on)"
      case MeasurementPeriod.LastMonth           => fr"time_bucket('1 day', occurred_on)"
    }

    val limit = period match {
      case MeasurementPeriod.LastTwentyFourHours => 24
      case MeasurementPeriod.LastSevenDays       => 7
      case MeasurementPeriod.LastTwoWeeks        => 14
      case MeasurementPeriod.LastMonth           => 31
    }

    (fr"SELECT $timeBucket AS \"on\", round(avg(battery))::int as battery, avg(temperature) as temperature, avg(humidity) as humidity, avg(lux) as lux, avg(soil_pf) as soil_pf FROM station_measurements WHERE station_id = $stationId GROUP BY 1 ORDER BY 1 ASC LIMIT $limit")
      .query[StationMeasurement]
      .to[List]
  }
}
