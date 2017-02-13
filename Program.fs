
(*  River v0.1
    Queue tweets by dropping images into a folder!

    By twitter.com/matnesis


    To do
        x Learn to Yaml (doesn't work on netcore)
        x Learn to tweet images/text
        - Learn to JSON :/
        - Scan current folder for images
        - CRUD a Yaml based on images found
        - Read from a Yaml with time to post
        - Post once to twitter the images with a time *)


open Tweetinvi
open System
open System.IO


(*  Twitter *)

type TweetData = {
    message : string;
    image: string;
    publish_date: string;
}

// Auth
let key = "w9HTFA5cCDPhmLqUYqrSK6kXK"
let keySecret = "kkrn0e66g5hpKzLwEbnvY6KVR4pqUfDnJNBvoXcgjqCvZGu1Bv"
let token = "55159989-NThkem3iDw1U1L3fEA0iLdwqYfeEkPHzLRfNPg4uI"
let tokenSecret = "DboMLBHwRBVW2QQmpEXPBbs1UEMIeFkHNvEgxJT4q8Z8Z"

Tweetinvi.Auth.SetUserCredentials(key, keySecret, token, tokenSecret) |> ignore

// Image upload
let image = File.ReadAllBytes((Directory.GetCurrentDirectory()) + "/test-images/primer-corruption.png")
let imageMedia = Upload.UploadImage(image)

// The tweet
let parameters = new Parameters.PublishTweetOptionalParameters()
parameters.Medias.Add(imageMedia)

let tweet = Tweet.PublishTweet("This tweet is a #test", parameters)


(*  MAIN *)
[<EntryPoint>]
let main argv =
    printfn "%A" argv
    printfn "%s" (Directory.GetCurrentDirectory())

    0 // Exit code