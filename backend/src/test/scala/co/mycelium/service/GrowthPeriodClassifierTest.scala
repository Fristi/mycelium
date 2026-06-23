package co.mycelium.service

import co.mycelium.domain.*
import weaver.*

import java.time.Instant

object GrowthPeriodClassifierTest extends FunSuite {

  private val variables = PlantProfileVariables(
    Interval(1500, 3400),
    Interval(100, 1000),
    Interval(18, 28),
    Interval(40, 70),
    Interval(15, 60),
    Interval(350, 2000)
  )

  private val profile = PlantProfile("Test Plant", variables)

  private def measurement(
      on: Instant,
      lux: Double = 500,
      temperature: Double = 22,
      humidity: Double = 55,
      soilMoisture: Double = 35
  ): StationMeasurement =
    StationMeasurement(on, 85, temperature, humidity, lux, soilMoisture)

  test("returns empty list when no profile") {
    val measurements = List(measurement(Instant.parse("2025-01-01T10:00:00Z")))
    expect(
      GrowthPeriodClassifier.classify(measurements, None, MeasurementPeriod.LastTwentyFourHours).isEmpty
    )
  }

  test("returns empty list when no measurements") {
    expect(
      GrowthPeriodClassifier.classify(Nil, Some(profile), MeasurementPeriod.LastTwentyFourHours).isEmpty
    )
  }

  test("classifies productive bucket") {
    val on = Instant.parse("2025-01-01T10:00:00Z")
    val result =
      GrowthPeriodClassifier.classify(
        List(measurement(on)),
        Some(profile),
        MeasurementPeriod.LastTwentyFourHours
      )
    expect.eql(result.length, 1) &&
    expect(result.head.kind == GrowthPeriodKind.Productive)
  }

  test("classifies heat stress") {
    val on = Instant.parse("2025-01-01T10:00:00Z")
    val result =
      GrowthPeriodClassifier.classify(
        List(measurement(on, temperature = 35)),
        Some(profile),
        MeasurementPeriod.LastTwentyFourHours
      )
    expect(result.head.kind == GrowthPeriodKind.NonProductive(NonProductiveReason.HeatStress))
  }

  test("classifies low light") {
    val on = Instant.parse("2025-01-01T10:00:00Z")
    val result =
      GrowthPeriodClassifier.classify(
        List(measurement(on, lux = 50)),
        Some(profile),
        MeasurementPeriod.LastTwentyFourHours
      )
    expect(result.head.kind == GrowthPeriodKind.NonProductive(NonProductiveReason.LowLight))
  }

  test("heat stress takes priority over low light") {
    val on = Instant.parse("2025-01-01T10:00:00Z")
    val result =
      GrowthPeriodClassifier.classify(
        List(measurement(on, lux = 50, temperature = 35)),
        Some(profile),
        MeasurementPeriod.LastTwentyFourHours
      )
    expect(result.head.kind == GrowthPeriodKind.NonProductive(NonProductiveReason.HeatStress))
  }

  test("classifies low soil moisture") {
    val on = Instant.parse("2025-01-01T10:00:00Z")
    val result =
      GrowthPeriodClassifier.classify(
        List(measurement(on, soilMoisture = 10)),
        Some(profile),
        MeasurementPeriod.LastTwentyFourHours
      )
    expect(result.head.kind == GrowthPeriodKind.NonProductive(NonProductiveReason.LowSoilMoisture))
  }

  test("classifies high soil moisture") {
    val on = Instant.parse("2025-01-01T10:00:00Z")
    val result =
      GrowthPeriodClassifier.classify(
        List(measurement(on, soilMoisture = 70)),
        Some(profile),
        MeasurementPeriod.LastTwentyFourHours
      )
    expect(result.head.kind == GrowthPeriodKind.NonProductive(NonProductiveReason.HighSoilMoisture))
  }

  test("humidity takes priority over low soil moisture") {
    val on = Instant.parse("2025-01-01T10:00:00Z")
    val result =
      GrowthPeriodClassifier.classify(
        List(measurement(on, humidity = 30, soilMoisture = 10)),
        Some(profile),
        MeasurementPeriod.LastTwentyFourHours
      )
    expect(result.head.kind == GrowthPeriodKind.NonProductive(NonProductiveReason.LowHumidity))
  }

  test("merges consecutive buckets with same classification") {
    val t0 = Instant.parse("2025-01-01T10:00:00Z")
    val t1 = Instant.parse("2025-01-01T10:15:00Z")
    val t2 = Instant.parse("2025-01-01T10:30:00Z")
    val measurements = List(
      measurement(t0, lux = 50),
      measurement(t1, lux = 60),
      measurement(t2, lux = 500)
    )
    val result =
      GrowthPeriodClassifier.classify(measurements, Some(profile), MeasurementPeriod.LastTwentyFourHours)
    expect.eql(result.length, 2) &&
    expect(result.head.kind == GrowthPeriodKind.NonProductive(NonProductiveReason.LowLight)) &&
    expect(result.head.start == t0) &&
    expect(result.head.end == t2) &&
    expect(result(1).kind == GrowthPeriodKind.Productive)
  }
}
