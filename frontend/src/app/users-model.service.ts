import { Injectable } from "@angular/core";
import { User, PaginatorParams, ResourceCollection } from "./model";
import { BehaviorSubject } from "rxjs";
import { HttpClient, HttpHeaders, HttpParams } from "@angular/common/http";

export interface UsersStore {
  users: User[];
  total: number;
}

@Injectable({
  providedIn: "root"
})
export class UsersModelService {
  private store: UsersStore = {
    users: [],
    total: 0
  };
  private subject: BehaviorSubject<UsersStore>;

  constructor(private http: HttpClient) {
    this.subject = new BehaviorSubject<UsersStore>(this.store);
  }

  get users() {
    return this.subject.asObservable();
  }

  select(paginatorParams: PaginatorParams) {
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

    let apiUri = `/api/v1/users`;
    return this.http.get(apiUri, options).subscribe(
      (collection: ResourceCollection<User>) => {
        this.store.users = collection.items;
        this.store.total = collection.total;

        this.subject.next(this.store);
      },
      err => {
        this.subject.error(err);
      }
    );
  }
}
