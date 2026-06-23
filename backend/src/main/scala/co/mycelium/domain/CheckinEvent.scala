package co.mycelium.domain

import io.circe.{Decoder, Encoder}
import io.circe.derivation.Configuration

import java.time.Instant

sealed trait CheckinEvent

object CheckinEvent:
  case class Measurement(
      start: Instant,
      end: Option[Instant],
      battery: Int,
      lux: Double,
      temperature: Double,
      humidity: Double,
      soilMoisture: Double
  ) extends CheckinEvent

  case class Watering(
      occurredAt: Instant,
      durationMsec: Long
  ) extends CheckinEvent

  private given Configuration = Configuration.default.withDiscriminator("_type")

  given Encoder.AsObject[CheckinEvent] = Encoder.AsObject.derivedConfigured
  given Decoder[CheckinEvent]          = Decoder.derivedConfigured
