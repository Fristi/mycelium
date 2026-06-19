package co.mycelium.domain

import io.circe.derivation.Configuration
import io.circe.{Decoder, Encoder}

import java.time.Instant

enum NonProductiveReason derives Encoder, Decoder:
  case HeatStress, LowLight, HighLight, LowHumidity, HighHumidity

sealed trait GrowthPeriodKind

object GrowthPeriodKind:
  case object Productive extends GrowthPeriodKind
  case class NonProductive(reason: NonProductiveReason) extends GrowthPeriodKind

  private given Configuration = Configuration.default.withDiscriminator("_type")

  given Encoder.AsObject[GrowthPeriodKind] = Encoder.AsObject.derivedConfigured
  given Decoder[GrowthPeriodKind]          = Decoder.derivedConfigured

final case class GrowthPeriod(
    start: Instant,
    end: Instant,
    kind: GrowthPeriodKind
) derives Encoder.AsObject,
      Decoder
