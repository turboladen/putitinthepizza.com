require 'sinatra'

get '/' do
  @audio_files = [
    { file: 'gimme_pizza', name: 'gimme pizza' },
    { file: 'hey_are_you_ready_to_play', name: 'are you ready?' },
    { file: 'p-i-z-z-a', name: 'p-i-z-z-a' },
    { file: 'um_did_i_happen_to_say', name: 'did i happen to say?' },
    { file: 'finger_lickin', name: "finger lickin'" },
    { file: 'whipped_cream', name: 'whipped cream pourin' },
    { file: 'pizza_pie', name: 'fly fly pizza pie' },
    { file: 'caramel', name: 'caramel coconut cream' },
    { file: 'spaghetti', name: '1 2 3 4 5 spaghetti' },
    { file: 'pasta', name: 'pasta, fishsticks, ketchup, meatloaf' },
    { file: 'uh', name: 'uhh... put it in the pizza' },
  ]

  erb :index
end

get '/fay_wrays' do
  erb :fay_wrays
end
