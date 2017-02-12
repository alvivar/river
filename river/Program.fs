
(*  River v0.1
    Queue tweets by dropping images into a folder!

    By twitter.com/matnesis


    To do
        x Learn to Yaml
        - Learn to tweet images/text
        - Scan current folder for images
        - CRUD a Yaml based on images found
        - Read from a Yaml with time to post
        - Post once to twitter the images with a time *)


open FSharp.Configuration


(*  YAML Config *)
[<Literal>] // This allows the bind at compile time
let YamlFile = "/Users/andresv/Projects/river/river/config.yaml" // By using this provider I can read yaml

type ConfigFile = YamlConfig<YamlFile>
let data = ConfigFile()
data.Load(YamlFile) // Load is important, at defining the provider only the schema gets loaded

// Sorting by date | F# Sequence to generic list
data.waiting <- new System.Collections.Generic.List<ConfigFile.waiting_Item_Type>(data.waiting |> Seq.sortBy (fun x -> x.date))
data.Save(YamlFile)


(*  Twitter *)
type TweetData = {
    message : string;
    image: string;
    publish_date: string;
}

let key = "w9HTFA5cCDPhmLqUYqrSK6kXK"
let secret = "kkrn0e66g5hpKzLwEbnvY6KVR4pqUfDnJNBvoXcgjqCvZGu1Bv"
let token = "55159989-NThkem3iDw1U1L3fEA0iLdwqYfeEkPHzLRfNPg4uI"
let tokenSecret = "DboMLBHwRBVW2QQmpEXPBbs1UEMIeFkHNvEgxJT4q8Z8Z"


(*  MAIN *)
[<EntryPoint>]
let main argv =
    printfn "%A" argv
    0 // Exit code
