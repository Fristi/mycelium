package co.mycelium.domain

import cats.Eq
import cats.derived.*
import org.typelevel.cats.time.*
import io.circe.{Decoder, Encoder}

import java.time.Instant

final case class StationMeasurement(
    on: Instant,
    battery: Int,
    temperature: Double,
    humidity: Double,
    lux: Double,
    soilPf: Double
) derives Encoder.AsObject,
      Decoder,
      Eq
