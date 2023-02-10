// #region imports
    // #region libraries
    import {
        PluridIconDocuments,
    } from '@plurid/plurid-icons-react';
    // #endregion libraries


    // #region external
    import Files from '~shared/kernel/components/Files';
    // #endregion external
// #endregion imports



// #region module
export const icons = {
    files: PluridIconDocuments,
};


export const dashboards = [
    {
        id: 'files',
        icon: icons.files,
        label: 'files',
        renderers: {
            'files': Files,
        },
    },
];
// #endregion module
