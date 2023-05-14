# Chapter 2: Web App

In this chapter, we will cover how to use the OpenRailwayMap Exporter web app to visualize and interact with the downloaded railway data in a browser environment.

## 2.1 Compiling to WebAssembly (WASM)

To run the web app, you'll first need to compile the OpenRailwayMap Exporter to WebAssembly (WASM) using the following command:

```sh
wasm-pack build --target web
```

This command will generate a `pkg` folder containing the compiled WASM files, JavaScript bindings, and other necessary files.

## 2.2 Running the Web App

After compiling the project to WASM, you can run the web app using a local web server. In this example, we'll use Python's built-in HTTP server. If you don't have Python installed, you can download it from the official Python website.

Run the following command to start the HTTP server:

```sh
python3 -m http.server
```

This command will start the server on port 8000 by default. If you want to use a different port, simply add the desired port number at the end of the command, like this:

```sh
python3 -m http.server 8080
```

## 2.3 Accessing the Web App in Your Browser

Once the HTTP server is running, open your browser and navigate to http://localhost:8000 (or replace 8000 with the port number you used). You should see the OpenRailwayMap Exporter web app in your browser.

From the web app, you can download railway data, visualize it on a map, and interact with the railway elements. You can also customize the appearance of the map, such as the colors and line styles, to better visualize the railway data.

In the next chapter, we'll explore the 3D visualization capabilities of the OpenRailwayMap Exporter.
