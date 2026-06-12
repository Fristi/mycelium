package co.mycelium.domain

import cats.effect.IO
import io.circe.parser.*
import weaver.SimpleIOSuite

import java.time.Instant

object CheckinEventCodecTest extends SimpleIOSuite:

  test("decodes externally tagged measurement events from edge-central client") {
    val json =
      """[
        |  {
        |    "_type": "Measurement",
        |    "start": "2023-11-14T22:13:20Z",
        |    "end": "2023-11-14T22:18:20Z",
        |    "battery": 85,
        |    "lux": 1200.5,
        |    "temperature": 22.3,
        |    "humidity": 55.0,
        |    "soilPf": 6.2
        |  }
        |]""".stripMargin

    IO.pure {
      val events = decode[List[CheckinEvent]](json).toOption.get
      val CheckinEvent.Measurement(start, end, battery, lux, temperature, humidity, soilPf) =
        events.head: @unchecked

      expect.all(
        events.size == 1,
        start == Instant.parse("2023-11-14T22:13:20Z"),
        end == Some(Instant.parse("2023-11-14T22:18:20Z")),
        battery == 85,
        lux == 1200.5,
        temperature == 22.3,
        humidity == 55.0,
        soilPf == 6.2
      )
    }
  }

  test("decodes externally tagged watering events from edge-central client") {
    val json =
      """[
        |  {
        |    "_type": "Watering",
        |    "occurredAt": "2023-11-14T22:23:20Z",
        |    "durationMsec": 15000
        |  }
        |]""".stripMargin

    IO.pure {
      val events = decode[List[CheckinEvent]](json).toOption.get
      val CheckinEvent.Watering(occurredAt, durationMsec) = events.head: @unchecked

      expect.all(
        events.size == 1,
        occurredAt == Instant.parse("2023-11-14T22:23:20Z"),
        durationMsec == 15000L
      )
    }
  }
