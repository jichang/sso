import { Injectable } from "@angular/core";
import { BehaviorSubject } from "rxjs";
import { HttpHeaders, HttpClient, HttpParams } from "@angular/common/http";
import { PaginatorParams, User, ResourceCollection } from "./model";

export interface Group {
  id: number;
  name: string;
  status: number;
}

export interface GroupUsers {
  offset: number;
  users: User[];
}

export interface GroupUsersCache {
  [index: number]: {
    total: number;
    pages: GroupUsers[];
  };
}

export interface GroupStore {
  groups: Group[];
  usersCache: GroupUsersCache;
}

@Injectable()
export class GroupModelService {
  private store: GroupStore;
  private subject: BehaviorSubject<Group[]>;
  private usersCacheSubject: BehaviorSubject<GroupUsersCache>;

  constructor(private http: HttpClient) {
    this.store = {
      groups: [],
      usersCache: {}
    };
    this.subject = new BehaviorSubject<Group[]>([]);
    this.usersCacheSubject = new BehaviorSubject<GroupUsersCache>(
      this.store.usersCache
    );
  }

  get groups() {
    return this.subject.asObservable();
  }

  get users() {
    return this.usersCacheSubject.asObservable();
  }

  select() {
    let headers = new HttpHeaders({
      "Content-Type": "application/json"
    });
    let options = {
      headers: headers
    };

    let apiUri = `/api/v1/groups`;
    return this.http.get(apiUri, options).subscribe(
      (groups: Group[]) => {
        this.store.groups = groups;
        this.subject.next(groups);
      },
      err => {
        this.subject.error(err);
      }
    );
  }

  selectUsers(groupId: number, paginatorParams: PaginatorParams) {
    let headers = new HttpHeaders({
      "Content-Type": "application/json"
    });
    let params = new HttpParams();
    params = params.append("limit", paginatorParams.limit.toString());
    params = params.append("offset", paginatorParams.offset.toString());
    let options = {
      headers,
      params
    };

    let apiUri = `/api/v1/groups/${groupId}/users`;
    return this.http.get(apiUri, options).subscribe(
      (users: ResourceCollection<User>) => {
        let groupUsers = {
          offset: paginatorParams.offset,
          users: users.items
        };
        let groupUsersCache = this.store.usersCache[groupId];
        if (groupUsersCache) {
          groupUsersCache.total = users.total;
          groupUsersCache.pages.unshift(groupUsers);
          groupUsersCache.pages.slice(0, 10);
        } else {
          this.store.usersCache[groupId] = {
            total: users.total,
            pages: [groupUsers]
          };
        }

        this.usersCacheSubject.next(this.store.usersCache);
      },
      err => {
        this.subject.error(err);
      }
    );
  }
}
