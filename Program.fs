
(*  River v0.1
    A bot that tweets at the best times when you drop images into a folder!

    By twitter.com/matnesis


    Working on
        x Learning to Yaml (.Configuration doesn't work on netcore)
        x Learning to tweet images/text
        x Scan and list current folder for images
        - Learn to JSON with Chiron
        - CRUD a config file based on images found
        - Chat though console to communicate with the bot
            - help | explain defaults and commands
            - scan <dir> | Analyzes a folder and create a config file with the defaults
            - tweet daily <#> | Set tweets per day
            - schedule su mo tu we th fr sa 7a 9: 7p 9:3 | Sets the manual schedule
            - sort 3 2 5, Sorts the queue
            - enable | play | activate
            - disable | stop | deactivate
            - exit | quit | die
            - schedule auto | Set auto schedule mode using tweets per day
        - Read from a config file with time to post
        - Waita and tweet withing the schedule
        - Best time to tweet
            - Followers, following analysis
            - Interactions analysis
            - Prediction model *)


open FSharp.Core
open System
open System.IO

open Tweetinvi
open Chiron


(*  Twitter *)

// type TweetData = { message : string; image: string; publish_date: string; }

// // Auth
// let key = "w9HTFA5cCDPhmLqUYqrSK6kXK"
// let keySecret = "kkrn0e66g5hpKzLwEbnvY6KVR4pqUfDnJNBvoXcgjqCvZGu1Bv"
// let token = "55159989-NThkem3iDw1U1L3fEA0iLdwqYfeEkPHzLRfNPg4uI"
// let tokenSecret = "DboMLBHwRBVW2QQmpEXPBbs1UEMIeFkHNvEgxJT4q8Z8Z"
// Tweetinvi.Auth.SetUserCredentials(key, keySecret, token, tokenSecret) |> ignore

// // Image upload
// let image = File.ReadAllBytes((Directory.GetCurrentDirectory()) + "/example/primer_corruption.png")
// let imageMedia = Upload.UploadImage(image)

// // The tweet
// let parameters = new Parameters.PublishTweetOptionalParameters()
// parameters.Medias.Add(imageMedia)
// let tweet = Tweet.PublishTweet("This tweet is a #test", parameters)


(* CHIRON *)

let formatExample =
        Object <| Map.ofList [
            "name", String "Marcus Griep"
            "isAdmin", Bool true
            "numbers", Array [ Number 1m; Number 2m; String "Fizz" ] ]

// let formatCompact = Json.format formatExample
// let formatPretty = Json.formatWith JsonFormattingOptions.Pretty formatExample

// printfn "%s\n" formatCompact
// printfn "%s\n" formatPretty


type Files =
    { files : string array }
    static member ToJson (x : Files) = json {
        do! Json.write "files" x.files
    }
    static member FromJson (_ : Files) = json {
        let! fs = Json.read "files"
        return { files = fs }
    }


// Returns a sequence with all files and folders.
let rec allFiles dirs =
    match dirs with
    | dirs when Seq.isEmpty dirs -> Seq.empty
    | _ -> seq { yield! dirs |> Seq.collect Directory.EnumerateFiles
                 yield! dirs |> Seq.collect Directory.EnumerateDirectories |> allFiles }


(*  MAIN *)
[<EntryPoint>]
let main argv =

    let dir = (Directory.GetCurrentDirectory())


    printfn "Arguments %A" argv
    printfn "Current directory %s\n" dir


    // Print all files and folders
    // [| dir |]
    //     |> allFiles
    //     |> Seq.iter (fun x -> printfn "> %s" x)

    let jfiles =
        { files = [| dir |] |> allFiles |> Array.ofSeq }
        |> Json.serialize
        |> Json.format

    // let flist = [| dir |] |> allFiles |> Array.ofSeq |> Json.m

    // let formatExample =
    //     Object <| Map.ofList [
    //         "files", flist ]

    // printfn "%s" jfiles


    0 // Exit code