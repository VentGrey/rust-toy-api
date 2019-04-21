module Main exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Http
import Json.Decode as Json
import Json.Encode as Encode


type alias Book =
    { title : String
    , author : String
    , published : Bool
    }

type alias Model =
    {books : List Book}



init =
    ( Model [], Cmd.none )

type Msg
    = GetBooks (Result Http.Error (List Book))
    | RequestBooks

update: Msg -> Model -> (Model, Cmd Msg)
update msg model =
        case msg of
            GetBooks (Ok json) ->
                ( { model | books = json  }, Cmd.none )
            GetBooks (Err e) ->
                ( Debug.log (toString e) model, Cmd.none )
            RequestBooks ->
                (model, getBooks)


getBooks : Cmd Msg
getBooks =
    let
        url =
            "http://localhost:8000/api/v1/books"

        req =
            Http.get url decodeBooks
    in
        Http.send GetBooks req
