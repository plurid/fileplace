// #region imports
    // #region libraries
    import React from 'react';

    import {
        PluridLink,
        PluridRouterLink,
        PluridReactComponent,
    } from '@plurid/plurid-react';
    // #endregion libraries


    // #region internal
    import {
        StyledPage,
    } from './styled';
    // #endregion internal
// #endregion imports



// #region module
const Page: PluridReactComponent<{}> = (
    properties,
) => {
    /** properties */
    // const {
    //     plurid,
    // } = properties;


    /** render */
    return (
        <StyledPage>
            <h1>
                fileplace viewer
            </h1>
        </StyledPage>
    );
}
// #endregion module



// #region exports
export default Page;
// #endregion exports
