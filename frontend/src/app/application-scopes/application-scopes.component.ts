import { Component, Input, OnInit } from "@angular/core";
import { Router, ActivatedRoute } from "@angular/router";
import { Scope, ScopeModelService } from "../scope-model.service";
import { session } from "../model";

@Component({
  selector: "application-scopes",
  templateUrl: "./application-scopes.component.html",
  styleUrls: ["./application-scopes.component.css"]
})
export class ApplicationScopesComponent implements OnInit {
  scopes: Scope[] = [];

  constructor(
    private router: Router,
    private route: ActivatedRoute,
    private scopeModel: ScopeModelService
  ) {}

  ngOnInit() {
    this.scopeModel.scopes.subscribe(scopes => {
      this.scopes = scopes;
    });

    let currUser = session.currUser();
    if (currUser) {
      this.scopeModel.select(
        currUser.id,
        this.route.parent.snapshot.params["id"]
      );
    } else {
      this.router.navigate(["signin"]);
    }
  }
}
