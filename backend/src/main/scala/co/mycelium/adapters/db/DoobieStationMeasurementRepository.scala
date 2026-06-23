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
      "insert into station_measurements (station_id, occurred_on, ended_on, battery, temperature, humidity, lux, soil_moisture) values (?, ?, ?, ?, ?, ?, ?, ?)"
    )
      .updateMany(
        measurements.map(m =>
          (stationId, m.start, m.end, m.battery, m.temperature, m.humidity, m.lux, m.soilMoisture)
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

    val lookback = MeasurementPeriod.lookbackInterval(period)

    // Daily buckets use peak lux / peak temperature so night readings do not
    // drag aggregates below profile thresholds when classifying growth periods.
    val aggregates = period match {
      case MeasurementPeriod.LastTwentyFourHours =>
        fr"round(avg(battery))::int as battery, avg(temperature) as temperature, avg(humidity) as humidity, avg(lux) as lux, avg(soil_moisture) as soil_moisture"
      case _ =>
        fr"round(avg(battery))::int as battery, max(temperature) as temperature, avg(humidity) as humidity, max(lux) as lux, avg(soil_moisture) as soil_moisture"
    }

    (fr"SELECT $timeBucket AS bucket_at, " ++ aggregates ++
      fr" FROM station_measurements WHERE station_id = $stationId AND occurred_on >= now() - interval " ++
      Fragment.const(s"'$lookback'") ++
      fr" GROUP BY 1 ORDER BY 1 ASC")
      .query[(java.time.Instant, Int, Double, Double, Double, Double)]
      .map { case (on, battery, temperature, humidity, lux, soilMoisture) =>
        StationMeasurement(on, battery, temperature, humidity, lux, soilMoisture)
      }
      .to[List]
  }
}
