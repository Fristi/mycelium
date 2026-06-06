package co.mycelium.endpoints

import cats.effect.IO
import co.mycelium.domain.*
import co.mycelium.service.StationService
import org.http4s.HttpRoutes
import sttp.capabilities.fs2.Fs2Streams
import sttp.model.headers.Origin
import sttp.tapir.*
import sttp.tapir.generic.Configuration
import sttp.tapir.generic.auto.*
import sttp.tapir.json.circe.*
import sttp.tapir.server.http4s.{Http4sServerInterpreter, Http4sServerOptions}
import sttp.tapir.server.interceptor.cors.{CORSConfig, CORSInterceptor}
import sttp.tapir.server.model.ValuedEndpointOutput

import java.time.Instant
import java.util.UUID
import scala.annotation.experimental
import scala.concurrent.duration.FiniteDuration

@experimental
object Stations extends TapirSchemas {

  val secured = endpoint
    .securityIn(auth.bearer[String]())
    .serverSecurityLogic(Auth.validate)

  object endpoints {
    val stationsNonSecured = endpoint.in("stations")
    val stationsSecured    = secured.in("stations")
    val profileSecured     = secured.in("profiles")

    val list = stationsSecured.get.out(jsonBody[List[Station]]).name("listStations")
    val add  =
      stationsSecured.post.in(jsonBody[StationInsert]).out(jsonBody[UUID]).name("addStation")
    val details = stationsSecured.get
      .in(path[UUID]("stationId"))
      .in(query[Option[MeasurementPeriod]]("period"))
      .name("getStation")
      .out(jsonBody[StationDetails])
    val update =
      stationsSecured.put
        .in(path[UUID]("stationId"))
        .in(jsonBody[StationUpdate])
        .name("updateStation")
    val delete  = stationsSecured.delete.in(path[UUID]("stationId")).name("deleteStation")
    val checkIn = stationsSecured
      .in(path[UUID]("stationId"))
      .in("checkin")
      .put
      .in(jsonBody[List[StationMeasurement]])
      .name("checkinStation")
      .out(jsonBody[Watering])
    val watered = stationsSecured
      .in(path[UUID]("stationId"))
      .in("watered")
      .post
      .in(jsonBody[Watering])
      .name("wateredAtStation")
    val log = stationsSecured
      .in(path[UUID]("stationId"))
      .in("log")
      .in(query[Option[Long]]("page"))
      .name("getStationLog")
      .out(jsonBody[List[StationLog]])

    val uploadAvatar = stationsSecured.post
      .in(path[UUID]("stationId"))
      .in("upload")
      .in(streamBody(Fs2Streams.apply[IO])(Schema.schemaForByte, CodecFormat.OctetStream()))
      .name("uploadAvatar")
      .out(jsonBody[List[PlantProfile]])

    val viewAvatar = stationsNonSecured.get
      .in(path[UUID]("stationId"))
      .in("avatar")
      .name("viewAvatar")
      .out(streamBody(Fs2Streams.apply[IO])(Schema.schemaForByte, CodecFormat.OctetStream()))

    val setProfile = profileSecured.post
      .in("profile")
      .in(path[UUID]("stationId"))
      .in(jsonBody[PlantProfile])
      .name("setProfile")
      .out(emptyOutput)

    val getProfiles = profileSecured.get
      .name("getProfiles")
      .out(jsonBody[List[StationPlantProfile]])

    val nonSecuredEndpoints = Set(viewAvatar)

    val securedEndpoints =
      Set(
        list,
        add,
        details,
        update,
        delete,
        checkIn,
        watered,
        log,
        uploadAvatar,
        setProfile,
        getProfiles
      )
  }

  def toMyceliumError(error: String): ValuedEndpointOutput[?] =
    ValuedEndpointOutput(jsonBody[MyceliumError], MyceliumError(error))

  val serverOptions = Http4sServerOptions
    .customiseInterceptors[IO]
    .defaultHandlers(toMyceliumError)
    .serverLog(
      Some(
        Http4sServerOptions
          .defaultServerLog[IO]
          .logWhenHandled(true)
          .logWhenReceived(true)
          .logAllDecodeFailures(true)
      )
    )
    .corsInterceptor(
      CORSInterceptor.customOrThrow[IO](
        CORSConfig.default.allowCredentials
          .allowOrigin(Origin.Host("http", "localhost", Some(1420)))
      )
    )
    .options

  def routes(svc: StationService[IO]): HttpRoutes[IO] = {

    val list    = endpoints.list.serverLogic(at => _ => svc.list(at.sub).map(Right(_)))
    val add     = endpoints.add.serverLogic(at => insert => svc.add(at.sub, insert).map(Right(_)))
    val delete  = endpoints.delete.serverLogic(at => id => svc.delete(at.sub, id).as(Right(())))
    val checkin = endpoints.checkIn.serverLogic(at =>
      (id, measurements) => svc.checkin(at.sub, id, measurements).map(Right(_))
    )
    val update = endpoints.update.serverLogic(at =>
      (id, update) => svc.update(at.sub, id, update).map(Right(_))
    )
    val details =
      endpoints.details.serverLogic(at => (id, period) => svc.details(at.sub, period, id))
    val watered = endpoints.watered.serverLogic(at =>
      (id, request) => svc.watered(at.sub, id, request).map(Right(_))
    )
    val log =
      endpoints.log.serverLogic(at => (id, page) => svc.getLogs(at.sub, id, page).map(Right(_)))

    val uploadAvatar =
      endpoints.uploadAvatar.serverLogic(at =>
        (id, stream) => svc.uploadAvatar(at.sub, id, stream).map(Right(_))
      )

    val viewAvatar =
      endpoints.viewAvatar.serverLogic(id => svc.viewAvatar(id).map(Right(_)))

    val setProfile =
      endpoints.setProfile.serverLogic(at =>
        (id, profile) => svc.setProfile(at.sub, id, profile).map(Right(_))
      )

    val getProfiles =
      endpoints.getProfiles.serverLogic(at => _ => svc.getProfiles(at.sub).map(Right(_)))

    Http4sServerInterpreter(serverOptions)
      .toRoutes(
        List(
          list,
          add,
          delete,
          log,
          watered,
          checkin,
          details,
          update,
          uploadAvatar,
          viewAvatar,
          setProfile,
          getProfiles
        )
      )
  }
}

trait TapirSchemas {
  implicit val customConfiguration: Configuration =
    Configuration.default.withDiscriminator("_type")

  implicit val schemaFiniteDuration: Schema[FiniteDuration] = Schema.string

  implicit def schemaForInterval[A: Schema]: Schema[Interval[A]] = Schema.derived[Interval[A]]

  implicit val codecMeasurementPeriod: Codec[String, MeasurementPeriod, CodecFormat.TextPlain] =
    Codec.string.map(
      Mapping.fromDecode((str: String) =>
        DecodeResult.fromOption(MeasurementPeriod.fromString(str))
      )(_.repr)
    )
}
