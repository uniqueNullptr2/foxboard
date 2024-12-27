export interface ProjectModel {
    name: string;
    id: string;
    columns: ProjectColumn[]
}

export interface ProjectColumn {
    name: string;
    id: string;
}

export interface Page<T> {
    total: number;
    count: number;
    items: T[]
}