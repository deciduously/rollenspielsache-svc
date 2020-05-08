
/*
get '/' do
'Congrats, you\'re on Rollenspielsache'
end

get '/ping' do
'pong'
end

get '/roll/:input' do
input = params['input']
roller = RollenspielsacheSvc::StringRoller.new(input)
json = roller.result.to_json
puts json
json
end
*/

fn main() {
    println!("Hello, world!");
}
