package co.mycelium.domain

import io.circe.{Decoder, Encoder}

import java.time.Instant

final case class StationWatering(
    occurredAt: Instant,
    durationMsec: Long
) derives Encoder.AsObject,
      Decoder
