const runner = require('@plurid/runner').default;

const Fileplace = require('../distribution').default;



runner(
    async (
        check,
    ) => {
        const fileplace = new Fileplace(
            'localhost:56165',
            '__TEST_MODE__',
            {
                log: true,
                secure: false,
            },
        );

        await new Promise((resolve) => {
            resolve();
        });
    },
);
