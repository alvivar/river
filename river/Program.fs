
open FSharp.Configuration
open FSharp.Data.Toolbox.Twitter
open System.Windows.Forms


(* YAML Config *)

[<Literal>] // This allows the bind at compile time
let YamlFile = "/Users/andresv/Projects/river/river/config.yaml" // By using this provider I can read yaml

type ConfigFile = YamlConfig<YamlFile>
let data = ConfigFile()
data.Load(YamlFile) // Load is important, at defining the provider only the schema gets loaded


// Sorting by date | F# Sequence to generic list
data.waiting <- new System.Collections.Generic.List<ConfigFile.waiting_Item_Type>(data.waiting |> Seq.sortBy (fun x -> x.date))
data.Save(YamlFile)


(* Twitter *)

type TweetData = {
    message : string;
    image: string;
    publish_date: string;
}

let key = "w9HTFA5cCDPhmLqUYqrSK6kXK"
let secret = "kkrn0e66g5hpKzLwEbnvY6KVR4pqUfDnJNBvoXcgjqCvZGu1Bv"
let token = "55159989-NThkem3iDw1U1L3fEA0iLdwqYfeEkPHzLRfNPg4uI"
let tokenSecret = "DboMLBHwRBVW2QQmpEXPBbs1UEMIeFkHNvEgxJT4q8Z8Z"
let twitcon = Twitter.AuthenticateAppSingleUser(key, secret, token, tokenSecret)

let tweet = twitcon.Tweets.Post("This tweet is a test")


(* MAIN *)

[<EntryPoint>]
let main argv =
    printfn "%A" argv
    0 // Exit code


// TODO
// - Read config files
// - Post to twitter image / text
// - Scans images
