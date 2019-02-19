import { Component, OnInit, OnDestroy } from "@angular/core";
import { Router } from "@angular/router";
import { Role, RoleModelService } from "../role-model.service";
import { session } from "../model";
import { Subscription } from "rxjs";
import { MatSnackBar } from "@angular/material";
import {
  Permission,
  PermissionModelService
} from "../permission-model.service";

@Component({
  selector: "roles-page",
  templateUrl: "./roles-page.component.html",
  styleUrls: ["./roles-page.component.css"]
})
export class RolesPageComponent implements OnInit, OnDestroy {
  roles: Role[] = [];
  permissions: Permission[] = [];
  rolesSubscription: Subscription;
  permissionsSubscription: Subscription;

  constructor(
    private router: Router,
    private roleModel: RoleModelService,
    private permissionModel: PermissionModelService,
    private snackBar: MatSnackBar
  ) {}

  ngOnInit() {
    this.rolesSubscription = this.roleModel.roles.subscribe(
      roles => {
        this.roles = roles;
      },
      err => {
        if (err.status === 403) {
          this.snackBar.open("action is forbidden", "Dismiss", {
            duration: 3000
          });
        }
      }
    );

    this.permissionsSubscription = this.permissionModel.permissions.subscribe(
      permissions => {
        this.permissions = permissions;
      },
      err => {
        if (err.status === 403) {
          this.snackBar.open("action is forbidden", "Dismiss", {
            duration: 3000
          });
        }
      }
    );

    let currUser = session.currUser();
    if (currUser) {
      this.roleModel.select();
      this.permissionModel.select();
    } else {
      this.router.navigate(["signin"]);
    }
  }

  ngOnDestroy() {
    this.rolesSubscription.unsubscribe();
    this.permissionsSubscription.unsubscribe();
  }

  grantPermission({
    role,
    permission
  }: {
    role: Role;
    permission: Permission;
  }) {
    this.roleModel.grantPermission(role, permission).subscribe(
      () => {
        this.snackBar.open("permission is granted", "Dismiss", {
          duration: 3000
        });
      },
      err => {
        this.snackBar.open("error occoured", "Dismiss", {
          duration: 3000
        });
      }
    );
  }

  revokePermission({
    role,
    permission
  }: {
    role: Role;
    permission: Permission;
  }) {
    this.roleModel.revokePermission(role, permission).subscribe(
      () => {
        this.snackBar.open("permission is revoked", "Dismiss", {
          duration: 3000
        });
      },
      err => {
        this.snackBar.open("error occoured", "Dismiss", {
          duration: 3000
        });
      }
    );
  }
}
