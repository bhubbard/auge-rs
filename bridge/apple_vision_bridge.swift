import Foundation
import Vision
import CoreImage
import CoreGraphics
import ImageIO

@available(macOS 26.0, *)
@main
struct AppleVisionBridgeMain {
    static func main() async {
        let args = ProcessInfo.processInfo.arguments
        guard args.count >= 3 else {
            fputs("Usage: apple_vision_bridge <mode> <image_path> [options_json]\n", stderr)
            exit(2)
        }

        let mode = args[1]
        let path = args[2]
        let url = URL(fileURLWithPath: path)

        guard let imageSource = CGImageSourceCreateWithURL(url as CFURL, nil),
              let cgImage = CGImageSourceCreateImageAtIndex(imageSource, 0, nil) else {
            fputs("error: could not load image from \(path)\n", stderr)
            exit(1)
        }

        let handler = VNImageRequestHandler(cgImage: cgImage, options: [:])

        do {
            switch mode {
            case "ocr":
                let request = VNRecognizeTextRequest()
                request.recognitionLevel = .accurate
                request.usesLanguageCorrection = true
                try handler.perform([request])
                let observations = request.results ?? []
                var lines: [[String: Any]] = []
                var fullText: [String] = []
                for obs in observations {
                    if let candidate = obs.topCandidates(1).first {
                        fullText.append(candidate.string)
                        lines.append([
                            "text": candidate.string,
                            "confidence": Double(candidate.confidence),
                            "x": Double(obs.boundingBox.origin.x),
                            "y": Double(obs.boundingBox.origin.y),
                            "width": Double(obs.boundingBox.size.width),
                            "height": Double(obs.boundingBox.size.height)
                        ])
                    }
                }
                let payload: [String: Any] = [
                    "text": fullText.joined(separator: "\n"),
                    "lines": lines
                ]
                printJSON(payload)

            case "classify":
                let request = VNClassifyImageRequest()
                try handler.perform([request])
                let observations = request.results ?? []
                let items: [[String: Any]] = observations.prefix(20).map { obs in
                    [
                        "label": obs.identifier,
                        "confidence": Double(obs.confidence)
                    ]
                }
                printJSON(["classifications": items])

            case "barcode":
                let request = VNDetectBarcodesRequest()
                try handler.perform([request])
                let observations = request.results ?? []
                let barcodes: [[String: Any]] = observations.compactMap { obs in
                    guard let payload = obs.payloadStringValue else { return nil }
                    return [
                        "payload": payload,
                        "symbology": obs.symbology.rawValue
                    ]
                }
                printJSON(["barcodes": barcodes])

            case "faces":
                let request = VNDetectFaceRectanglesRequest()
                try handler.perform([request])
                let observations = request.results ?? []
                let faces: [[String: Any]] = observations.map { obs in
                    [
                        "x": Double(obs.boundingBox.origin.x),
                        "y": Double(obs.boundingBox.origin.y),
                        "width": Double(obs.boundingBox.size.width),
                        "height": Double(obs.boundingBox.size.height)
                    ]
                }
                printJSON(["faces": faces])

            case "humans":
                let request = VNDetectHumanRectanglesRequest()
                try handler.perform([request])
                let observations = request.results ?? []
                let humans: [[String: Any]] = observations.map { obs in
                    [
                        "x": Double(obs.boundingBox.origin.x),
                        "y": Double(obs.boundingBox.origin.y),
                        "width": Double(obs.boundingBox.size.width),
                        "height": Double(obs.boundingBox.size.height),
                        "confidence": Double(obs.confidence),
                        "upper_body_only": obs.upperBodyOnly
                    ]
                }
                printJSON(["humans": humans])

            case "rectangles":
                let request = VNDetectRectanglesRequest()
                try handler.perform([request])
                let observations = request.results ?? []
                let rects: [[String: Any]] = observations.map { obs in
                    [
                        "top_left": ["x": Double(obs.topLeft.x), "y": Double(obs.topLeft.y)],
                        "top_right": ["x": Double(obs.topRight.x), "y": Double(obs.topRight.y)],
                        "bottom_left": ["x": Double(obs.bottomLeft.x), "y": Double(obs.bottomLeft.y)],
                        "bottom_right": ["x": Double(obs.bottomRight.x), "y": Double(obs.bottomRight.y)],
                        "confidence": Double(obs.confidence)
                    ]
                }
                printJSON(["rectangles": rects])

            case "horizon":
                let request = VNDetectHorizonRequest()
                try handler.perform([request])
                if let obs = request.results?.first {
                    let rad = Double(obs.angle)
                    let deg = rad * 180.0 / .pi
                    printJSON([
                        "angle_radians": rad,
                        "angle_degrees": deg
                    ])
                } else {
                    printJSON(["angle_radians": 0.0, "angle_degrees": 0.0])
                }

            case "animals":
                let request = VNRecognizeAnimalsRequest()
                try handler.perform([request])
                let observations = request.results ?? []
                let animals: [[String: Any]] = observations.map { obs in
                    let top = obs.labels.first
                    return [
                        "label": top?.identifier ?? "animal",
                        "confidence": Double(top?.confidence ?? obs.confidence),
                        "x": Double(obs.boundingBox.origin.x),
                        "y": Double(obs.boundingBox.origin.y),
                        "width": Double(obs.boundingBox.size.width),
                        "height": Double(obs.boundingBox.size.height)
                    ]
                }
                printJSON(["animals": animals])

            default:
                fputs("Unsupported mode: \(mode)\n", stderr)
                exit(2)
            }
        } catch {
            fputs("error: \(error.localizedDescription)\n", stderr)
            exit(1)
        }
    }

    static func printJSON(_ dict: [String: Any]) {
        if let data = try? JSONSerialization.data(withJSONObject: dict, options: []),
           let str = String(data: data, encoding: .utf8) {
            print(str)
        }
    }
}
