//
//  Network.swift
//  Botify
//
//  Created by Patrick Lemiuex on 4/28/24.
//

import Foundation
import Apollo
import Foundation
import BotifyApi
import ApolloSQLite

class Network {
    static let shared = Network()
    private(set) lazy var apollo = ApolloClient(networkTransport: self.transport, store: self.store)

    private lazy var store: ApolloStore = {
        let url = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first!.appendingPathComponent("apollo")
        let cache = try! SQLiteNormalizedCache(fileURL: url)
        return ApolloStore(cache: cache)
    }()

    private lazy var transport: RequestChainNetworkTransport = {
        let url = URL(string: "http://localhost:8081/graphql")!
        return RequestChainNetworkTransport(interceptorProvider: DefaultInterceptorProvider(store: self.store),
                                            endpointURL: url)
    }()

}
