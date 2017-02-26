
(*  River v0.1
    A bot that tweets at the best times when you drop images into a folder!

    By twitter.com/matnesis


    Working on
        x Learning to Yaml (.Configuration doesn't work on netcore)
        x Learning to tweet images/text
        x Scan and list current folder for images
        x Learn to JSON with Chiron
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

// 'json' is a computation expression by Chiron to define a serializable file.
type ConfigFile =
    { dailyTweets: int
      files : string array }
    static member ToJson (x : ConfigFile) = json {
        do! Json.write "files" x.files
        do! Json.write "dailyTweets" x.dailyTweets
    }
    static member FromJson (_ : ConfigFile) = json {
        let! fs = Json.read "files"
        let! dt = Json.read "dailyTweets"
        return { files = fs
                 dailyTweets = dt }
    }


// Returns the default configuration.
let defaultConfig =
    { dailyTweets = 5
      files = [||] }


// Returns all files and folders.
let rec allFiles dirs =
    match dirs with
    | dirs when Seq.isEmpty dirs -> Seq.empty
    | _ -> seq { yield! dirs |> Seq.collect Directory.EnumerateFiles
                 yield! dirs |> Seq.collect Directory.EnumerateDirectories |> allFiles }


(*  MAIN *)
[<EntryPoint>]
let main argv =

    // Files
    let dir = (Directory.GetCurrentDirectory())

    let cfgName = "config.json";
    let cfgFile = Path.Combine [| dir ; cfgName |]


    // Header
    printfn "Arguments %A" argv
    printfn "Current directory %s\n" dir


    // Read or create the config file
    let cfgTxt =
        if File.Exists cfgFile
        then File.ReadAllText cfgFile
        else defaultConfig |> Json.serialize |> Json.format

    do File.WriteAllText(cfgFile, cfgTxt)

    // Parse the current config
    let cfg : ConfigFile = cfgTxt |> Json.parse |> Json.deserialize


    // Prints the config Json
    let jsn =
        cfg
        |> Json.serialize
        |> Json.formatWith JsonFormattingOptions.Pretty

    printfn "%s" jsn


    // All files and directories
    let alfs = [| dir |] |> allFiles |> Array.ofSeq


    0 // Exit code