import { Component, Input, Output, EventEmitter } from "@angular/core";
import { Authorization } from "../authorization-model.service";

export type AuthorizationActionType = "revoke";

export interface AuthorizationAction {
  type: AuthorizationActionType;
  authorization: Authorization;
}

@Component({
  selector: "authorizations-list",
  templateUrl: "./authorizations-list.component.html",
  styleUrls: ["./authorizations-list.component.css"]
})
export class AuthorizationsListComponent {
  @Input() authorizations: Authorization[];
  @Output() action = new EventEmitter<AuthorizationAction>();

  revoke(authorization: Authorization) {
    this.action.emit({
      type: "revoke",
      authorization
    });
  }
}
