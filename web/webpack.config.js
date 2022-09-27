const path = require("path");
const webpack = require('webpack');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const HtmlWebPackPlugin = require('html-webpack-plugin');
const BundleAnalyzerPlugin = require('webpack-bundle-analyzer').BundleAnalyzerPlugin;

const packageFolder = path.resolve(__dirname, 'build')
const isDevelopment = process.env.NODE_ENV !== "production"

module.exports = {
    mode: isDevelopment ? 'development' : 'production',
    devtool: isDevelopment ? 'source-map' : false,

    watchOptions: {
        poll: 1000,
        aggregateTimeout: 1000,
        ignored: ['**/node_modules']
    },

    entry: path.resolve(__dirname, "src", "index.tsx"),

    output: {
        path: packageFolder,
        sourceMapFilename: '[file].map',
        filename: `assets/js/[name].min.js`,
    },

    resolve: {
        extensions: ['.tsx', '.ts', '.jsx', '.js', '.scss', '.css'],
        modules: ['node_modules'],
    },

    module: {
        rules: [
            {
                test: /\.(t|j)sx?$/,
                exclude: /node_modules/,
                use: {
                    loader: "babel-loader",
                    options: {
                        presets: [
                            // https://babeljs.io/docs/en/babel-preset-env
                            "@babel/preset-env",
                            // https://babeljs.io/docs/en/babel-preset-typescript
                            "@babel/preset-typescript",
                            // https://babeljs.io/docs/en/babel-preset-react
                            ["@babel/preset-react", { development: isDevelopment }],
                        ],
                        plugins: [isDevelopment && require.resolve('react-refresh/babel')].filter(Boolean),
                    }
                }
            },
            {
                test: /\.s?[ac]ss$/i,
                use: [
                    isDevelopment ? 'style-loader' :
                        {
                            // save the css to external file
                            loader: MiniCssExtractPlugin.loader,
                            options: {
                                esModule: false
                            },
                        },
                    {
                        // becombine other css files into one
                        // https://www.npmjs.com/package/css-loader
                        loader: 'css-loader',
                        options: {
                            esModule: false,
                            importLoaders: 2, // 2 other loaders used first, postcss-loader and sass-loader
                            sourceMap: isDevelopment,
                        }
                    },
                    {
                        // process tailwind stuff
                        // https://webpack.js.org/loaders/postcss-loader/
                        loader: "postcss-loader",
                        options: {
                            sourceMap: isDevelopment,
                            postcssOptions: {
                                plugins: [
                                    require("tailwindcss"),
                                    // add addtional postcss plugins here
                                    // easily find plugins at https://www.postcss.parts/
                                ]
                            }
                        },
                    },
                    {
                        // load sass files into css files
                        loader: 'sass-loader',
                        options: {
                            sourceMap: isDevelopment
                        }
                    },
                ],
            },
            {
                test: /\.html$/i,
                loader: "html-loader",
                options: {
                    esModule: false,
                },
            },
            {
                test: /\.(png|svg|jpg|gif)$/,
                loader: 'file-loader',
                options: {
                    name: 'assets/img/[name].[ext]',
                    // outputPath: "images",
                    esModule: false,
                },
            },
            {
                test: /\.(ttf|eot|otf|woff)$/,
                loader: 'file-loader',
                options: {
                    name: 'assets/fonts/[name].[ext]',
                    esModule: false,
                },
            },
            {
                test: /\.(ico)$/,
                loader: 'file-loader',
                options: {
                    name: '[name].[ext]',
                    esModule: false,
                },
            }
        ],
    },

    plugins: [
        new webpack.ProvidePlugin({
            React: "react",
        }),

        // build html file
        new HtmlWebPackPlugin({
            template: "./src/index.html",
            filename: "./index.html"
        }),

        // https://webpack.js.org/plugins/mini-css-extract-plugin/
        // dump css into its own files
        new MiniCssExtractPlugin({
            filename: `assets/css/[name].min.css`,
        }),

        // Bundle Analyzer, a visual view of what goes into each JS file.
        // https://www.npmjs.com/package/webpack-bundle-analyzer
        process.env.ANALYZE && new BundleAnalyzerPlugin()

    ].filter(Boolean),

    // https://webpack.js.org/configuration/dev-server/
    devServer: {
        port: 3031,
        host: '0.0.0.0',
        compress: true,
        allowedHosts: 'all',
        hot: true
    },

    performance: {
        hints: false,
        maxEntrypointSize: 512000,
        maxAssetSize: 512000
    }
}