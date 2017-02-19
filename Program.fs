
(*  River v0.1
    Queue tweets by dropping images into a folder!

    By twitter.com/matnesis


    TO DO
        x Learn to Yaml (doesn't work on netcore)
        x Learn to tweet images/text
        - Learn to JSON via Chiron
        - Scan and list current folder for images
        - CRUD a config file based on images found
        - Read from a config file with time to post
        - Chat though console to communicate with the bot
            - Explain defaults and commands
            - Set tweets per day
            - Set auto schedule mode based on tweets per day
            - Sort queue
            - Set fixed schedule mode
            - Exit
        - Best time to tweet
            - Followers, following analysis
            - Interactions analysis
            - Prediction model
        - Post once to twitter withing the time scheduled *)


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

// let formatExample =
//         Object <| Map.ofList [
//             "name", String "Marcus Griep"
//             "isAdmin", Bool true
//             "numbers", Array [ Number 1m; Number 2m; String "Fizz" ] ]

// let formatCompact = Json.format formatExample
// let formatPretty = Json.formatWith JsonFormattingOptions.Pretty formatExample

// printfn "%s\n" formatCompact
// printfn "%s\n" formatPretty



// Returns all files and folders.
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
    [| dir |] |> allFiles |> Seq.iter (fun x -> printfn "%s" x)


    0 // Exit code