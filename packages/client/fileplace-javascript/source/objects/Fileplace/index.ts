// #region imports
    // #region libraries
    import fetch from 'cross-fetch';
    // #endregion libraries


    // #region external
    import {
        FileplaceOptions,
        FileplaceMetadata,
    } from '~data/interfaces';
    // #endregion external
// #endregion imports



// #region module
class Fileplace {
    #host: string;
    #token: string;
    #owner: string;

    constructor(
        host: string,
        token: string = '',
        options?: Partial<FileplaceOptions>,
    ) {
        this.#host = host;
        this.#token = token;
        this.#owner = options?.owner ? `&owner=${options.owner}` : '';
    }


    private async fileOperation(
        place: string,
        name: string,
        type: 'get' | 'metadata' | 'remove',
    ) {
        place = encodeURIComponent(place);
        name = encodeURIComponent(name);
        const path = `/${type}?place=${place}&name=${name}` + this.#owner;

        const query = await fetch(
            this.#host + path,
            {
                headers: {
                    'Fileplace-Token': this.#token,
                },
            },
        );

        return query;
    }


    /**
     * Get file stream if it exists.
     *
     * @param place
     * @param name
     */
    public async get(
        place: string,
        name: string,
    ): Promise<ReadableStream<Uint8Array> | undefined> {
        const query = await this.fileOperation(place, name, 'get');

        if (
            query.status !== 200
            || !query.body
        ) {
            return;
        }

        return query.body;
    }

    /**
     * Get file metadata.
     *
     * @param place
     * @param name
     * @returns
     */
    public async metadata(
        place: string,
        name: string,
    ): Promise<FileplaceMetadata | undefined> {
        const query = await this.fileOperation(place, name, 'metadata');

        if (query.status !== 200) {
            return;
        }

        const response = await query.json();
        return response as FileplaceMetadata;
    }

    /**
     * Store file stream in a place.
     *
     * If no name is given a random name will be used and returned.
     *
     * @param place
     * @param stream
     * @param name
     */
    public async store(
        place: string,
        stream: NodeJS.ReadStream,
        name?: string,
    ): Promise<string | undefined> {
        return;
    }

    /**
     * Remove file from place.
     *
     * @param place
     * @param name
     * @returns
     */
    public async remove(
        place: string,
        name: string,
    ): Promise<boolean> {
        const query = await this.fileOperation(place, name, 'remove');

        if (query.status !== 200) {
            return false;
        }

        return true;
    }

    /**
     * Check if `place` or `file` exists
     *
     * @param name
     * @param type
     */
    public async exists(
        name: string,
        type: 'place' | 'file' = 'file',
    ): Promise<boolean> {
        return false;
    }
}
// #endregion module



// #region exports
export default Fileplace;
// #endregion exports
