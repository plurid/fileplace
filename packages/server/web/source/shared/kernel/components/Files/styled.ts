// #region imports
    // #region libraries
    import styled from 'styled-components';

    import {
        Theme,
    } from '@plurid/plurid-themes';
    // #region libraries
// #region imports



// #region module
export interface IStyledFiles {
    theme: Theme;
}

export const StyledFiles = styled.div<IStyledFiles>`
    padding: 2rem;
    height: 100%;
`;
// #region module
