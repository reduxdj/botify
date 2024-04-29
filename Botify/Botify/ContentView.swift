import SwiftUI
import Kingfisher

struct ContentView: View {
    @StateObject var songsViewModel = SongsViewModel()
    @State private var selectedSong: Song?

    var body: some View {
        NavigationView {
            List(songsViewModel.songs, id: \.title) { song in
                Button(action: {
                    self.selectedSong = song
                }) {
                    SongRow(song: song)
                        .frame(width: 150, height: 80)
                        .padding()
                }
            }
            .navigationTitle("Songs")
            .onAppear {
                songsViewModel.loadSongs()
            }
            .alert(isPresented: $songsViewModel.isErrorPresent) {
                Alert(title: Text("Error"), message: Text(songsViewModel.errorMessage ?? "Unknown error"), dismissButton: .default(Text("OK")))
            }

            
            if let selectedSong = selectedSong {
                SongDetailView(song: selectedSong)
            } else {
                Text("Select a song")
                    .frame(maxWidth: .infinity, maxHeight: .infinity)
                    .background(Color(.systemGray))
            }
        }
    }
}
struct SongDetailView: View {
    var song: Song

    var body: some View {
        VStack(alignment: .leading, spacing: 20) {
            KFImage(URL(string: song.imageUrl))
                .resizable()
                .scaledToFit()
                .frame(width: 350, height: 350)
                .cornerRadius(5)
            Text(song.title)
                .font(.largeTitle)

            Text(song.artist)
                .font(.title2)

            Spacer()
        }
        .padding()
        .navigationTitle(song.title)
    }
}

struct SongRow: View {
    let song: Song

    var body: some View {
        HStack {
            KFImage(URL(string: song.imageUrl))
                .resizable()
                .scaledToFit()
                .frame(width: 50, height: 50)
                .cornerRadius(5)

            VStack(alignment: .leading) {
                Text(song.title)
                    .font(.headline)
                Text(song.artist)
                    .font(.subheadline)
                    .foregroundColor(.secondary)
            }
        }
    }
}


// SwiftUI Preview
struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
