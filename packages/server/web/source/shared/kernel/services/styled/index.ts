// #region imports
    // #region libraries
    import {
        pluridal,
        universal,
    } from '@plurid/plurid-ui-components-react';
    // #endregion libraries
// #endregion imports



// #region module
const {
    containers: {
        DashboardsRenderer: PluridDashboardsRenderer,
        EntityView: PluridEntityView,
    },
} = pluridal;

const {
    varia: {
        CopyableLine: PluridCopyableLine,
    },
} = universal;
// #endregion module



// #region exports
export {
    PluridDashboardsRenderer,
    PluridEntityView,

    PluridCopyableLine,
};
// #endregion exports
