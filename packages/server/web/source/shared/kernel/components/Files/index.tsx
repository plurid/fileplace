// #region imports
    // #region libraries
    import React, {
        useRef,
        useState,
        useEffect,
    } from 'react';

    import {
        AnyAction,
        ThunkDispatch,
    } from '@reduxjs/toolkit';
    import { connect } from 'react-redux';

    import {
        Theme,
    } from '@plurid/plurid-themes';

    import {
        PluridIconDelete,
        PluridIconInfo,
    } from '@plurid/plurid-icons-react';

    import {
        PluridLink,
    } from '@plurid/plurid-react';

    import {
        EntityViewRefAttributes,
    } from '@plurid/plurid-ui-components-react';

    import {
        uuid,
    } from '@plurid/plurid-functions';
    // #endregion libraries


    // #region external
    import {
        PluridEntityView,
        PluridCopyableLine,
    } from '~kernel-services/styled';

    import {
        getPlaces,
        getPlaceFiles,
    } from '~kernel-services/logic/files';

    import { AppState } from '~kernel-services/state/store';
    import StateContext from '~kernel-services/state/context';
    import selectors from '~kernel-services/state/selectors';
    // import actions from '~kernel-services/state/actions';
    // #endregion external


    // #region internal
    import {
        StyledFiles,
    } from './styled';
    // #endregion internal
// #endregion imports



// #region module
export interface RenderFile {
    id: string;
    name: string;
    link: string;
    generated_at: number;
    pluridlink: string;
    type: 'file' | 'place';
}


export const rowsHeader = (
    <>
        <div />

        <div>
            name
        </div>

        <div>
            generated on
        </div>

        <div />

        <div />
    </>
);

export const rowTemplate = '20px 0.5fr 0.5fr 60px 60px';

export const rowRenderFields = [
    'link', 'name', 'generatedAt', 'pluridlink', 'obliterate',
];


export const renderDate = (
    value: any,
) => {
    if (!value) {
        return '';
    }

    return new Date(value).toLocaleString();
}

export const abstractRowRenderer = (
    columns: string[],
    data: Record<string, any>,
    methods: Record<string, any>,
) => {
    const renderColumns: JSX.Element[] = [];

    for (const column of columns) {
        const key = Math.random() + '';

        switch (column) {
            case 'link':
                renderColumns.push(
                    <PluridCopyableLine
                        key={key}
                        data={data.link}
                        viewData=" "
                        copyMessage=" "
                    />
                );
                break;
            case 'name':
                renderColumns.push(
                    <div
                        key={key}
                    >
                        {data.name}
                    </div>
                );
                break;
            case 'generatedAt':
                renderColumns.push(
                    <div
                        key={key}
                    >
                        {renderDate(data.generated_at)}
                    </div>
                );
                break;
            case 'pluridlink':
                renderColumns.push(
                    <PluridLink
                        key={key}
                        route={data.pluridlink}
                        devisible={true}
                        style={{
                            display: 'grid',
                            placeContent: 'center',
                        }}
                    >
                        <PluridIconInfo
                            atClick={() => {}}
                        />
                    </PluridLink>
                );
                break;
            case 'obliterate':
                renderColumns.push(
                    <PluridIconDelete
                        key={key}
                        atClick={() => methods.handleObliterate(data.id)}
                    />
                );
                break;
        }
    }

    return (
        <>
            {renderColumns}
        </>
    );
}


export const composeRenderFiles = (
    data: Record<string, string[]>,
    root: string,
) => {
    const files: RenderFile[] = [];

    if (root === '/') {
        for (const key of Object.keys(data)) {
            if (!key.includes('/')) {
                const file: RenderFile = {
                    id: uuid.multiple(),
                    name: key,
                    generated_at: Date.now(),
                    link: '',
                    pluridlink: '',
                    type: 'place',
                };
                files.push(file);
            }
        }
    } else {
        Object.entries(data).forEach(
            ([key, placeFiles]) => {
                if (key === root) {
                    for (const filename of placeFiles) {
                        const file: RenderFile = {
                            id: uuid.multiple(),
                            name: filename,
                            generated_at: Date.now(),
                            link: '',
                            pluridlink: '',
                            type: 'file',
                        };
                        files.push(file);
                    }
                }
            },
        );
    }

    return files;
}



export interface FilesOwnProperties {
}

export interface FilesStateProperties {
    stateGeneralTheme: Theme;
    stateInteractionTheme: Theme;
}

export interface FilesDispatchProperties {
}

export type FilesProperties =
    & FilesOwnProperties
    & FilesStateProperties
    & FilesDispatchProperties;


const Files: React.FC<FilesProperties> = (
    properties,
) => {
    // #region properties
    const {
        // #region state
        stateGeneralTheme,
        stateInteractionTheme,
        // #endregion state
    } = properties;
    // #endregion properties


    // #region references
    const entityView = useRef<EntityViewRefAttributes | null>(null);
    // #endregion references


    // #region handlers
    const handleObliterate = async (
        id: string,
    ) => {
        try {

        } catch (error) {
            return;
        }
    }

    const filterUpdate = (
        rawValue: string,
    ) => {
    }

    const actionScrollBottom = async (
        data: any[],
    ) => {
    }
    // #endregion handlers


    // #region state
    const [
        loading,
        setLoading,
    ] = useState(false);

    const [
        filterValue,
        setFilterValue,
    ] = useState('');

    const [
        filterIDs,
        setFilterIDs,
    ] = useState<string[]>([]);

    const [
        root,
        setRoot,
    ] = useState('/');

    const [
        files,
        setFiles,
    ] = useState<RenderFile[]>([]);
    // #endregion state


    // #region effects
    useEffect(() => {
        const load = async () => {
            const places = await getPlaces();
            const data = {};

            for (const place of places) {
                const files = await getPlaceFiles(place);
                data[place] = files;
            }

            const renderFiles = composeRenderFiles(data, root);
            setFiles(renderFiles);
        }

        load();
    }, []);
    // #endregion effects


    // #region render
    return (
        <StyledFiles
            theme={stateGeneralTheme}
        >
            <PluridEntityView
                ref={entityView}

                entities={files}
                searchFields={['name']}

                generalTheme={stateGeneralTheme}
                interactionTheme={stateInteractionTheme}

                rowTemplate={rowTemplate}
                rowsHeader={rowsHeader}
                noRows="no files"

                abstractRowRenderer={abstractRowRenderer}
                rowRenderFields={rowRenderFields}
                rowRenderMethods={{
                    handleObliterate,
                }}

                loading={loading ? 1 : 0}

                filterUpdate={filterUpdate}
                refresh={() => {
                }}

                actionButtonText=""
                actionButtonClick={() => {
                }}

                actionScrollBottom={actionScrollBottom}
            />
        </StyledFiles>
    );
    // #endregion render
}


const mapStateToProperties = (
    state: AppState,
): FilesStateProperties => ({
    stateGeneralTheme: selectors.themes.getGeneralTheme(state),
    stateInteractionTheme: selectors.themes.getInteractionTheme(state),
});


const mapDispatchToProperties = (
    dispatch: ThunkDispatch<{}, {}, AnyAction>,
): FilesDispatchProperties => ({
});


const ConnectedFiles = connect(
    mapStateToProperties,
    mapDispatchToProperties,
    null,
    {
        context: StateContext,
    },
)(Files);
// #endregion module



// #region exports
export default ConnectedFiles;
// #endregion exports
