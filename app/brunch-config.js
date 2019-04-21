exports.config = {
    files: {
        javascripts: {
            joinTo: "js/app.js"
        },
        stylesheets: {
            joinTo: "css/app.css"
        }
    },
    conventions: {
        assets: /^(static)/
    },
    paths: {
        watched: [
            "static", "css", "js", "vendor", "elm"
        ],
        public: "../public"
    }
}
