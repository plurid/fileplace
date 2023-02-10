// #region imports
    // #region libraries
    import React, {
        useEffect,
    } from 'react';

    import {
        PluridLink,
        PluridRouterLink,
        PluridReactComponent,
    } from '@plurid/plurid-react';
    // #endregion libraries


    // #region external
    import {
        getPlaces,
        getPlaceFiles,
    } from '~kernel-services/logic/files';

    import Files from '~kernel-components/Files';
    // #endregion external


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


    // #region effects
    useEffect(() => {
        const load = async () => {
            // const places = await getPlaces();
            // const oneFiles = await getPlaceFiles('one/two');
        }

        load();
    }, []);
    // #endregion effects


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
