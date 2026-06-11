package co.mycelium.domain

import io.circe.{Decoder, Encoder}

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
      soilPf: Double
  ) extends CheckinEvent

  case class Watering(
      occurredAt: Instant,
      durationMsec: Long
  ) extends CheckinEvent

  given Encoder.AsObject[CheckinEvent] = Encoder.AsObject.derived
  given Decoder[CheckinEvent]          = Decoder.derived
