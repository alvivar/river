
(*  River v0.1
    A bot that tweets at the best times when you drop images into a folder!

    By twitter.com/matnesis


    Working on
        x Learning to Yaml (.Configuration doesn't work on netcore)
        x Learning to tweet images/text
        x Scan and list current folder for images
        x Learn to JSON with Chiron
        x CRUD a config file
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
        - Wait and tweet withing the schedule
        - Best time to tweet
            - Followers, following analysis
            - Interactions analysis
            - Prediction model *)


open FSharp.Core
open System
open System.IO

open Tweetinvi
open Chiron


let R = System.Random()


(* Twitter *)

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


(* Chiron *)

// Computation expression that defines a serializable Json Chiron type.
type ConfigFile =
    { isActive : bool
      dailyTweets : int
      filesPending : string array
      filesSent : string array }
    static member ToJson (x : ConfigFile) = json {
        do! Json.write "isActive" x.isActive
        do! Json.write "dailyTweets" x.dailyTweets
        do! Json.write "filesPending" x.filesPending
        do! Json.write "filesSent" x.filesPending }
    static member FromJson (_ : ConfigFile) = json {
        let! at = Json.read "isActive"
        let! dt = Json.read "dailyTweets"
        let! fp = Json.read "filesPending"
        let! fs = Json.read "filesSent"
        return { isActive = at
                 dailyTweets = dt
                 filesPending = fp
                 filesSent = fs } }


// Returns the default configuration.
let defaultConfig =
    { isActive = false
      dailyTweets = 3
      filesPending = [||]
      filesSent = [||] }


(* Chat system *)

// Active record for commands.
let (|Help|Enable|Disable|Exit|None|) input =
    match input with
    | "help" | "h" -> Help
    | "enable" | "activate" | "on" -> Enable
    | "disable" | "deactivate" | "off" -> Disable
    | "exit" | "quit" | "die" -> Exit
    | _ -> None

// Active record for actions.
let (|Scan|Schedule|Sort|None|) input =
    match input with
    | "scan" | "analyze" -> Scan
    | "schedule" -> Schedule
    | "sort" | "order" -> Sort
    | _ -> None


// Emoticons
let positiveEmoticon =
    match R.Next(3) with
    | 0 -> ":D"
    | 1 -> ":)"
    | 2 -> ":P"
    | _ -> ":?"

let negativeEmoticon =
    match R.Next(3) with
    | 0 -> ":("
    | 1 -> ":O"
    | 2 -> ":C"
    | _ -> ":?"

// Returns a positive affirmative answer.
let positiveAnswer =
    match R.Next(2) with
    | 0 -> "Ok " + positiveEmoticon
    | 1 -> "Done " + positiveEmoticon
    | _ -> ":?"


// Hi
let salute =
    match R.Next(2) with
    | 0 -> "Hi! " + positiveEmoticon
    | 1 -> "Hello " + positiveEmoticon
    | 2 -> "Hey " + positiveEmoticon
    | _ -> ":?"


// All actions and commands!
let help =
    positiveAnswer

let enable =
    positiveAnswer

let disable =
    positiveAnswer

let exit =
    positiveAnswer

let scan =
    positiveAnswer

let schedule =
    positiveAnswer

let sort =
    positiveAnswer


// Executes the action and responds!
let chatResponse input =
    match input with
    | Help -> help
    | Enable -> enable
    | Disable -> disable
    | Exit -> exit
    | Scan -> scan
    | Schedule -> schedule
    | Sort -> sort
    | None -> ":?"


// Returns all files and folders from files and folders.
let rec allFiles dirs =
    match dirs with
    | dirs when Seq.isEmpty dirs -> Seq.empty
    | _ -> seq { yield! dirs |> Seq.collect Directory.EnumerateFiles
                 yield! dirs |> Seq.collect Directory.EnumerateDirectories |> allFiles }


(* Main *)
[<EntryPoint>]
let main argv =

    // Files
    let dir = (Directory.GetCurrentDirectory())

    let cfgName = "config.json";
    let cfgFile = Path.Combine [| dir ; cfgName |]


    (* Header *)

    printfn "River v0.1 | The best twitter bot ever\n"
    printfn "Arguments %A" argv
    printfn "Current directory %s\n" dir
    printfn "What's up?\n"


    (* Config *)

    // Read the file or default
    let cfgTxt =
        if File.Exists cfgFile
        then File.ReadAllText cfgFile
        else defaultConfig |> Json.serialize |> Json.format

    // Update the file
    do File.WriteAllText(cfgFile, cfgTxt)

    // Parse it
    let config : ConfigFile = cfgTxt |> Json.parse |> Json.deserialize

    // Print it
    let json =
        config
        |> Json.serialize
        |> Json.formatWith JsonFormattingOptions.Pretty
    printfn "Current config.js \n%s" json


    // All files and directories
    let allfs = [| dir |] |> allFiles |> Array.ofSeq


    0 // Exit code