// swift-tools-version: 5.9

import PackageDescription

let package = Package(
    name: "Nexus",
    platforms: [
        .macOS(.v13),
        .iOS(.v16),
    ],
    products: [
        .library(
            name: "Nexus",
            targets: ["Nexus"]
        )
    ],
    targets: [
        .target(
            name: "Nexus",
            dependencies: [
                "NexusRust"
            ],
            path: "apps/nexus/nexus"
        ),

        .binaryTarget(
            name: "NexusRust",
            path: "apps/nexus/Frameworks/Nexus.xcframework"
        ),
    ]
)
