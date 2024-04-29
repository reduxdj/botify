// @generated
// This file was automatically generated and should not be edited.

@_exported import ApolloAPI

public class GetSongsQuery: GraphQLQuery {
  public static let operationName: String = "getSongs"
  public static let operationDocument: ApolloAPI.OperationDocument = .init(
    definition: .init(
      #"query getSongs { getSongs { __typename title artist createdOn lastModified active mediaUrl imageUrl } }"#
    ))

  public init() {}

  public struct Data: BotifyApi.SelectionSet {
    public let __data: DataDict
    public init(_dataDict: DataDict) { __data = _dataDict }

    public static var __parentType: ApolloAPI.ParentType { BotifyApi.Objects.Query }
    public static var __selections: [ApolloAPI.Selection] { [
      .field("getSongs", [GetSong].self),
    ] }

    public var getSongs: [GetSong] { __data["getSongs"] }

    /// GetSong
    ///
    /// Parent Type: `Song`
    public struct GetSong: BotifyApi.SelectionSet {
      public let __data: DataDict
      public init(_dataDict: DataDict) { __data = _dataDict }

      public static var __parentType: ApolloAPI.ParentType { BotifyApi.Objects.Song }
      public static var __selections: [ApolloAPI.Selection] { [
        .field("__typename", String.self),
        .field("title", String.self),
        .field("artist", String.self),
        .field("createdOn", Int.self),
        .field("lastModified", Int.self),
        .field("active", Bool.self),
        .field("mediaUrl", String.self),
        .field("imageUrl", String.self),
      ] }

      public var title: String { __data["title"] }
      public var artist: String { __data["artist"] }
      public var createdOn: Int { __data["createdOn"] }
      public var lastModified: Int { __data["lastModified"] }
      public var active: Bool { __data["active"] }
      public var mediaUrl: String { __data["mediaUrl"] }
      public var imageUrl: String { __data["imageUrl"] }
    }
  }
}
