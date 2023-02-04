import esbuild from 'esbuild';



const isProduction = process.env.ENV_MODE === 'production';

const allPackagesExternalPlugin = () => ({
    name: 'all-packages-external',
    setup(build) {
        const filter = /^([a-z]|[@])/;

        build.onResolve(
            { filter },
            args => {
                return { path: args.path, external: true };
            },
        );
    },
});


const config = {
    entryPoints: [
        './source/index.ts',
    ],
    outdir: './distribution',
    platform: 'node',
    target: 'node18',
    format: 'esm',
    bundle: true,
    minify: isProduction ? true : false,
    external: [],
    plugins: [
        allPackagesExternalPlugin(),
    ],
};


esbuild
    .build(config)
    .catch((error) => {
        console.log(error);
        process.exit(1);
    });
