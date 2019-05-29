import { Injectable } from "@angular/core";
import { BehaviorSubject } from "rxjs";
import { map } from "rxjs/operators";
import { session } from "./model";
import { HttpClient, HttpHeaders } from "@angular/common/http";

export interface ClientSeret {
  Plaintext: string;
}

export interface Application {
  id?: number;
  name: string;
  website_uri: string;
  client_id?: string;
  client_secret?: ClientSeret;
  callback_uri: string;
  status: number;
}

export interface ApplicationStore {
  applications: Application[];
}

@Injectable()
export class ApplicationModelService {
  private store: ApplicationStore;
  private subject: BehaviorSubject<Application[]>;

  constructor(private http: HttpClient) {
    this.store = {
      applications: []
    };
    this.subject = new BehaviorSubject<Application[]>([]);
  }

  get applications() {
    return this.subject.asObservable();
  }

  select(userId: number) {
    let headers = new HttpHeaders({
      "Content-Type": "application/json"
    });
    let options = {
      headers: headers
    };

    let apiUri = `/api/v1/users/${userId}/applications`;
    this.http.get(apiUri, options).subscribe(
      (applications: Application[]) => {
        this.store.applications = applications;
        this.subject.next(applications);
      },
      err => {
        this.subject.error(err);
      }
    );
  }

  create(application: Application) {
    let headers = new HttpHeaders({
      "Content-Type": "application/json"
    });
    let options = {
      headers: headers
    };

    let apiUri = "/api/v1/users/" + session.currUser().id + "/applications";
    return this.http.post(apiUri, application, options).pipe(
      map((application: Application) => {
        this.store.applications.push(application);
        this.subject.next(Object.assign({}, this.store).applications);

        return application;
      })
    );
  }

  remove(applicatin: Application) {
    let apiUri = `/api/v1/users/${session.currUser().id}/applications/${
      applicatin.id
    }`;

    return this.http.delete(apiUri).pipe(
      map((application: Application) => {
        let index = this.store.applications.findIndex(
          _application => _application.id === application.id
        );
        this.store.applications.splice(index, 1);

        this.subject.next(Object.assign({}, this.store).applications);

        return application;
      })
    );
  }
}
