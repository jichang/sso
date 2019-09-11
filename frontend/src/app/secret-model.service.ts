import { Injectable } from "@angular/core";
import { HttpClient, HttpHeaders } from "@angular/common/http";
import { Observable, BehaviorSubject } from "rxjs";
import { session } from "./model";
import { map } from "rxjs/operators";

export interface Secret {
  id?: number;
  client_id: string;
  client_secret: number;
  status: number;
}

export interface SecretStore {
  secrets: Secret[];
}

@Injectable()
export class SecretModelService {
  private store: SecretStore;
  private subject: BehaviorSubject<Secret[]>;

  constructor(private http: HttpClient) {
    this.store = {
      secrets: []
    };
    this.subject = new BehaviorSubject<Secret[]>([]);
  }

  get secrets() {
    return this.subject.asObservable();
  }

  select(userId: number, applicationId: number) {
    let headers = new HttpHeaders({
      "Content-Type": "application/json"
    });
    let options = {
      headers: headers
    };

    let apiUri = `/api/v1/users/${userId}/applications/${applicationId}/secrets`;
    this.http.get(apiUri, options).subscribe((secrets: Secret[]) => {
      this.store.secrets = secrets;
      this.subject.next(secrets);
    });
  }

  create(userId: number, applicationId: number) {
    let headers = new HttpHeaders({
      "Content-Type": "application/json"
    });
    let options = {
      headers: headers
    };

    let apiUri = `/api/v1/users/${userId}/applications/${applicationId}//secrets`;
    return this.http.post(apiUri, options).pipe(
      map((secret: Secret) => {
        this.store.secrets.push(secret);
        this.subject.next(Object.assign({}, this.store).secrets);

        return secret;
      })
    );
  }

  remove(userId: number, applicationId: number, secret: Secret) {
    let headers = new HttpHeaders({
      "Content-Type": "application/json"
    });
    let options = {
      headers: headers
    };

    let apiUri = `/api/v1/users/${userId}/applications/${applicationId}//secrets/${
      secret.id
      }`;
    return this.http.delete(apiUri, options).pipe(
      map((secret: Secret) => {
        this.store.secrets = this.store.secrets.filter(
          _secret => _secret.id !== secret.id
        );
        this.subject.next(Object.assign({}, this.store).secrets);

        return secret;
      })
    );
  }
}
