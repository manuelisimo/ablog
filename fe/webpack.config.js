const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const path = require('path');

module.exports = {
    plugins: [new MiniCssExtractPlugin({
        filename: 'blog.css'
    })],
    entry: './blog.js',
    output: {
        filename: 'blog.js',
        path: path.resolve(__dirname, '../static'),
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