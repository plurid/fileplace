// #region imports
    // #region libraries
    import fetch from 'cross-fetch';
    // #endregion libraries


    // #region external
    import {
        FILEPLACE_STORE_ENDPOINT,
    } from '~shared/data/constants';
    // #endregion external
// #endregion imports



// #region module
export const getPlaces = async () => {
    try {
        const response = await fetch(
            FILEPLACE_STORE_ENDPOINT + '/places',
        );
        const data = await response.json();

        return data.places;
    } catch (error) {
        return [];
    }
}


export const getPlaceFiles = async (
    place: string,
) => {
    try {
        const response = await fetch(
            FILEPLACE_STORE_ENDPOINT + `/all?place=${encodeURIComponent(place)}`,
        );
        const data = await response.json();

        return data.files;
    } catch (error) {
        return [];
    }
}
// #endregion module
