const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const path = require('path');

let mode = 'development';
if ('NODE_ENV' in process.env) {
    mode = process.env.NODE_ENV;
}

let destination = path.resolve(__dirname, '../static');
// if (mode === 'production') {
//     destination = path.resolve(__dirname, 'dist');
// }

module.exports = {
    mode,
    plugins: [new MiniCssExtractPlugin({
        filename: 'blog.css'
    })],
    entry: './blog.js',
    output: {
        filename: 'blog.js',
        path: destination,
    },
    module: {
        rules: [
            {
                test: /\.styl$/,
                use: [{
                    loader: MiniCssExtractPlugin.loader,
                }, 'css-loader', 'stylus-loader'],
            },
            {
                test: /\.ttf$|\.woff2$|\.woff$/,
                use: [{
                    loader: 'url-loader',
                    options: {
                        limit: 8192,
                    },
                }],
            },
        ]
    }
};