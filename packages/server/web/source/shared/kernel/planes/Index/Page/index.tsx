// #region imports
    // #region libraries
    import React from 'react';

    import {
        AnyAction,
        ThunkDispatch,
    } from '@reduxjs/toolkit';
    import { connect } from 'react-redux';


    import {
        Theme,
    } from '@plurid/plurid-themes';

    import {
        PluridPlaneComponentProperty,
    } from '@plurid/plurid-react';
    // #endregion libraries


    // #region external
    import fileplaceLogo from '../../../assets/fileplace-logo.png';

    import {
        PluridDashboardsRenderer,
    } from '~kernel-services/styled';

    import { AppState } from '~kernel-services/state/store';
    import StateContext from '~kernel-services/state/context';
    import selectors from '~kernel-services/state/selectors';
    // import actions from '~kernel-services/state/actions';
    // #endregion external


    // #region internal
    import {
        dashboards,
    } from './data';

    import {
        StyledPage,
    } from './styled';
    // #endregion internal
// #endregion imports



// #region module
export interface PageOwnProperties {
    plurid: PluridPlaneComponentProperty;
}

export interface PageStateProperties {
    stateGeneralTheme: Theme;
    stateInteractionTheme: Theme;
}

export interface PageDispatchProperties {
}

export type PageProperties =
    & PageOwnProperties
    & PageStateProperties
    & PageDispatchProperties;


const Page: React.FC<PageProperties> = (
    properties,
) => {
    // #region properties
    const {
        // #region own
        plurid,
        // #endregion own

        // #region state
        stateGeneralTheme,
        // stateInteractionTheme,
        // #endregion state
    } = properties;

    const stateIdentonym = '';
    const stateCompactSelectors = false;

    const {
        activeDashboard,
        activeRender,
        fullRenderArea,
    } = plurid.plane.query;
    // #endregion properties


    // #region render
    return (
        <StyledPage
            theme={stateGeneralTheme}
        >
            <PluridDashboardsRenderer
                dashboards={dashboards}
                theme={stateGeneralTheme}

                activeDashboard={activeDashboard || 'files'}
                activeRender={activeRender}
                fullRenderArea={typeof fullRenderArea === 'undefined' ? undefined : fullRenderArea === 'true'}
                compactSelectors={stateCompactSelectors}
                rendererID={plurid.plane.planeID}
                identonym={stateIdentonym}
                brandingName="fileplace"
                brandingNameStyle={{
                    fontWeight: 'bolder',
                }}
                brandingLogo={fileplaceLogo}
                noDashboardRender={<>select a dashboard</>}

                atDashboardChange={(newDashboard: string) => {}}
                openManual={() => {
                    window.open('https://github.com/plurid/fileplace', '_blank');
                }}

                atUIChange={(
                    type,
                    value,
                ) => {
                    if (type === 'compactSelectors') {
                        // dispatchSetCompactSelectors(value);
                    }
                }}
            />
        </StyledPage>
    );
    // #endregion render
}


const mapStateToProperties = (
    state: AppState,
): PageStateProperties => ({
    stateGeneralTheme: selectors.themes.getGeneralTheme(state),
    stateInteractionTheme: selectors.themes.getInteractionTheme(state),
});


const mapDispatchToProperties = (
    dispatch: ThunkDispatch<{}, {}, AnyAction>,
): PageDispatchProperties => ({
});


const ConnectedPage = connect(
    mapStateToProperties,
    mapDispatchToProperties,
    null,
    {
        context: StateContext,
    },
)(Page);
// #endregion module



// #region exports
export default ConnectedPage;
// #endregion exports
