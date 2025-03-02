# edge_detector_wasm

## Build

```shell
$ wasm-pack build --target bundler
```

## Usage

```shell
$ npm install /path/to/edge_detector_wasm/pkg
```

```jsx
import {
  EdgeDetectionMethod,
  process_image_wasm,
  init,
} from "edge_detector_wasm";
```
