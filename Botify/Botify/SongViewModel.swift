import Foundation
import Apollo
import Combine
import BotifyApi

struct Song {
    let title: String
    let artist: String
    let imageUrl: String

}

class SongsViewModel: ObservableObject {
    @Published var songs: [Song] = [] 
    @Published var isLoading = false
    @Published var errorMessage: String?
    @Published var isErrorPresent = false

    private var cancellables: Set<AnyCancellable> = []

    init() {
        loadSongs()
    }

    func loadSongs() {
        self.isLoading = true
        self.errorMessage = nil
        Network.shared.apollo.fetch(query: GetSongsQuery(), cachePolicy: .returnCacheDataElseFetch) { [weak self] result in
            DispatchQueue.main.async {
                self?.isLoading = false
                switch result {
                case .success(let graphQLResult):
                if let songsData = graphQLResult.data?.getSongs.compactMap({ $0 }) {
                    self?.songs = songsData.map {
                        Song(title: $0.title, artist: $0.artist, imageUrl: $0.imageUrl)
                    }
                } else if let errors = graphQLResult.errors {
                    self?.errorMessage = errors.map { $0.localizedDescription }.joined(separator: "\n")
                    self?.isErrorPresent = true
                }
                case .failure(let error):
                    self?.errorMessage = error.localizedDescription
                    self?.isErrorPresent = true
                }
            }
        }
    }
}
